import { useEffect, useState } from "react";
import React from "react";
import { Redirect } from "react-router-dom";
import { $signUpDone, clearSignUp, signUpFx } from "./state";
import { useStore } from "effector-react";
import { Box, Paper, Stack, TextField, Typography } from "@mui/material";
import { LoadingButton } from "@mui/lab";

const SignUp = () => {
  const signUpDone = useStore($signUpDone);
  const isLoading = useStore(signUpFx.pending);

  const [nickname, setNickname] = useState("");
  const [name, setName] = useState("");
  const [password, setPassword] = useState("");
  const [confirmation, setConfirmation] = useState("");

  useEffect(() => {
    return () => {
      clearSignUp();
    };
  }, []);

  if (signUpDone) {
    return <Redirect to="/login" />;
  }

  const handleSubmit = (e) => {
    e.preventDefault();
    signUpFx({ nickname, name, password, confirmation });
  };

  return (
    <div className="flex items-center justify-center h-full">
      <Paper className="mx-auto text-center p-12" elevation={5}>
        <Typography variant="h3" component="h1">
          Welcome, Sempai :)
        </Typography>
        <Typography variant="h4" component="h2">
          First time? Let's sign up
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
              label="How to call you, Sempai?"
              variant="outlined"
              value={name}
              onChange={(e) => {
                setName(e.target.value);
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
            <TextField
              label="Confirmation"
              variant="outlined"
              type="password"
              value={confirmation}
              onChange={(e) => {
                setConfirmation(e.target.value);
              }}
            />
            <LoadingButton
              loading={isLoading}
              variant="contained"
              type="submit"
            >
              Sign up
            </LoadingButton>
          </Stack>
        </Box>
      </Paper>
    </div>
  );
};

export default SignUp;
