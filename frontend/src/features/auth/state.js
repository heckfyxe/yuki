import { createEffect, createEvent, createStore } from "effector";
import axios from "axios";

export const signUpFx = createEffect(async (payload) => {
  let response = await axios.post("http://localhost:8000/sign_up", payload);
  return response.data;
});

export const loginFx = createEffect(async (payload) => {
  let response = await axios.post("http://localhost:8000/login", payload);
  return response.data;
});

export const clearSignUp = createEvent();

export const $signUpDone = createStore(false)
  .on(signUpFx.done, () => true)
  .reset(signUpFx, clearSignUp);

export const $user = createStore(null).on(loginFx.doneData, (_, user) => user);

$user.watch((user) => {
  console.log(user);
});
