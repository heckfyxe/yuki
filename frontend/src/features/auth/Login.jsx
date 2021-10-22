import { useState } from "react";
import React from "react";
import { loginFx } from "./state";
import { Box, Paper, Stack, TextField, Typography } from "@mui/material";
import { LoadingButton } from "@mui/lab";
import { useStore } from "effector-react";

const Login = () => {
  const isLoading = useStore(loginFx.pending);

  const [nickname, setNickname] = useState("");
  const [password, setPassword] = useState("");

  const handleSubmit = (e) => {
    e.preventDefault();
    loginFx({ nickname, password });
  };

  return (
    <div className="flex items-center justify-center h-full">
      <Paper className="mx-auto text-center p-12" elevation={5}>
        <Typography variant="h4" component="h1">
          Oh, you back? Let's login
        </Typography>
        <br />
        <Box component="form" noValidate onSubmit={handleSubmit}>
          <Stack spacing={2}>
            <TextField
              label="Nickname"
              variant="outlined"
              value={nickname}
              onChange={(e) => {
                setNickname(e.target.value);
              }}
            />
            <TextField
              label="Password"
              variant="outlined"
              type="password"
              value={password}
              onChange={(e) => {
                setPassword(e.target.value);
              }}
            />
            <LoadingButton
              loading={isLoading}
              variant="contained"
              type="submit"
            >
              Login
            </LoadingButton>
          </Stack>
        </Box>
      </Paper>
    </div>
  );
};

export default Login;
