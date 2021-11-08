import { createEffect, createStore } from "effector";
import axios from "axios";

export const contentFx = createEffect(async (payload) => {
  let response = await axios.get("http://localhost:8001/content", {
    withCredentials: true,
  });
  return response.data;
});

export const $content = createStore("Not Loaded").on(
  contentFx.doneData,
  (_, payload) => payload
);

$content.watch((content) => {
  console.log(content);
});
