use hyper::service::{make_service_fn, service_fn};
use hyper::{http, Body, Client, Error, Method, Request, Response, Server, StatusCode, Uri};
use std::io::Read;

use cookie::{Cookie, CookieBuilder, CookieJar, Key, SameSite};
use hyper::header::{
    ACCESS_CONTROL_ALLOW_CREDENTIALS, ACCESS_CONTROL_ALLOW_HEADERS, ACCESS_CONTROL_ALLOW_METHODS,
    ACCESS_CONTROL_ALLOW_ORIGIN, AUTHORIZATION, COOKIE, ORIGIN, SET_COOKIE,
};
use hyper::http::uri::InvalidUri;
use hyper::http::HeaderValue;
use hyper_tls::HttpsConnector;

use std::net::SocketAddr;

use hyper::body::{aggregate, Buf};

use lazy_static::lazy_static;
use std::str::FromStr;
use thiserror::Error;
use time::{Duration, OffsetDateTime};

lazy_static! {
    static ref PORT: String = dotenv::var("BFF_PORT").unwrap();
    static ref FRONTEND_PORT: String = dotenv::var("PORT").unwrap();
    static ref API_PORT: String = dotenv::var("API_PORT").unwrap();
    static ref COOKIE_SECRET: String = dotenv::var("COOKIE_SECRET").unwrap();
}

#[derive(Debug, Error)]
pub enum ProxyError {
    #[error("Invalid uri")]
    InvalidUri(InvalidUri),
    #[error("Internal server error")]
    InternalError,
    #[error("Forward header error")]
    ForwardHeaderError,
    #[error("Missing origin header")]
    MissingOrigin,
    #[error("Not allowed origin")]
    NotAllowedOrigin,
    #[error("No session in cookies")]
    NoSession,
    #[error("No authorized")]
    NoAuthorized,
}

impl From<hyper::Error> for ProxyError {
    fn from(_: Error) -> Self {
        ProxyError::InternalError
    }
}

impl From<std::io::Error> for ProxyError {
    fn from(_: std::io::Error) -> Self {
        ProxyError::InternalError
    }
}

const NO_SESSION_PATHS: [&str; 2] = ["/sign_up", "/login"];

fn csrf(req: &Request<Body>) -> Result<(), ProxyError> {
    if req.method() != Method::GET && req.method() != Method::OPTIONS {
        match req.headers().get(ORIGIN) {
            Some(origin) => {
                let origin = origin.to_str().map_err(|_| ProxyError::NotAllowedOrigin)?;
                if origin != format!("http://localhost:{}", *FRONTEND_PORT) {
                    return Err(ProxyError::MissingOrigin);
                }
            }
            None => return Err(ProxyError::NotAllowedOrigin),
        }
    }
    Ok(())
}

/// Returns token
async fn token_from_req(req: &Request<Body>) -> Result<String, ProxyError> {
    return match req.headers().get(COOKIE) {
        Some(cookies) => {
            let token = extract_token(cookies).ok_or(ProxyError::NoSession)?;
            Ok(token)
        }
        None => Err(ProxyError::NoSession),
    };
}

fn parse_cookie(cookie: &str) -> CookieJar {
    let mut jar = CookieJar::new();
    cookie.split("; ").for_each(|entry| {
        jar.add_original(Cookie::from_str(entry).unwrap());
    });
    jar
}

fn extract_token(cookies: &HeaderValue) -> Option<String> {
    let cookies = cookies.to_str().ok()?;
    let jar = parse_cookie(cookies);
    let key = Key::from(COOKIE_SECRET.as_bytes());
    jar.private(&key)
        .get("session")
        .map(|cookies| cookies.value().to_string())
}

/// Returns session cookie
fn create_session_cookie(token: &str) -> String {
    let key = Key::from(COOKIE_SECRET.as_bytes());
    let mut jar = CookieJar::new();
    let cookie = CookieBuilder::new("session", token.to_string())
        .secure(true)
        .http_only(true)
        .same_site(SameSite::None)
        .expires(OffsetDateTime::now_utc() + Duration::days(1))
        .finish();
    jar.private_mut(&key).add(cookie);
    let cookie = jar.get("session").unwrap();
    cookie.to_string()
}

async fn login_response(response: Response<Body>) -> Result<Response<Body>, ProxyError> {
    let status = response.status();
    return match status {
        StatusCode::OK => {
            let mut token = String::new();
            aggregate(response)
                .await?
                .reader()
                .read_to_string(&mut token)?;
            let cookie = create_session_cookie(&token);
            let response = cors_response_builder(Response::builder())
                .status(status)
                .header(SET_COOKIE, cookie)
                .body(Body::empty())
                .unwrap();
            Ok::<_, ProxyError>(response)
        }
        _ => {
            let response = add_extra(response);
            Ok::<_, ProxyError>(response)
        }
    };
}

fn add_extra(resp: Response<Body>) -> Response<Body> {
    cors_response_builder(Response::builder())
        .status(resp.status())
        .body(resp.into_body())
        .unwrap()
}

fn cors_response_builder(resp: http::response::Builder) -> http::response::Builder {
    resp.header(ACCESS_CONTROL_ALLOW_ORIGIN, "http://localhost:3000")
        .header(
            ACCESS_CONTROL_ALLOW_METHODS,
            "POST, GET, PUT, DELETE, PATCH",
        )
        .header(ACCESS_CONTROL_ALLOW_HEADERS, "Content-Type")
        .header(ACCESS_CONTROL_ALLOW_CREDENTIALS, "true")
}

fn forward_uri(uri: &Uri) -> Uri {
    let authority = format!("127.0.0.1:{}", *API_PORT);
    Uri::builder()
        .scheme("http")
        .authority(authority)
        .path_and_query(uri.path_and_query().unwrap().clone())
        .build()
        .unwrap()
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let addr = SocketAddr::from(([127, 0, 0, 1], PORT.parse().unwrap()));

    let connector = HttpsConnector::new();
    let client = Client::builder().build(connector);

    let make_svc = make_service_fn(move |_| {
        let client = client.clone();
        async move {
            Ok::<_, ProxyError>(service_fn(move |mut req: Request<Body>| {
                let client = client.clone();
                async move {
                    csrf(&req)?;

                    if req.method() == Method::OPTIONS {
                        return Ok::<_, ProxyError>(
                            cors_response_builder(Response::builder())
                                .status(StatusCode::NO_CONTENT)
                                .body(Body::empty())
                                .unwrap(),
                        );
                    }

                    if !NO_SESSION_PATHS.contains(&req.uri().path()) {
                        let token = token_from_req(&req).await?;
                        req.headers_mut().insert(
                            AUTHORIZATION,
                            HeaderValue::from_str(format!("Bearer {}", token).as_str()).unwrap(),
                        );
                    }

                    *req.uri_mut() = forward_uri(req.uri());
                    let path = req.uri().path().to_string();
                    let response = client.request(req).await?;

                    if path == "/login" {
                        return login_response(response).await;
                    }

                    let response = add_extra(response);
                    Ok::<_, ProxyError>(response)
                }
            }))
        }
    });

    let server = Server::bind(&addr).serve(make_svc);
    server.await.unwrap()
}
