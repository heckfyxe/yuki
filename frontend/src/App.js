import SignUp from "features/auth/SignUp";
import React from "react";
import { BrowserRouter, Route, Switch } from "react-router-dom";
import "./App.css";
import Login from "./features/auth/Login";
import { Content } from "./features/content/Content";

function App() {
  return (
    <BrowserRouter>
      <div className="App">
        <Switch>
          <Route exact path="/sign_up">
            <SignUp />
          </Route>
          <Route exact path="/login">
            <Login />
          </Route>
          <Route exact="/">
            <Content />
          </Route>
        </Switch>
      </div>
    </BrowserRouter>
  );
}

export default App;
