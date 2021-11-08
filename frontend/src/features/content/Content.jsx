import { useStore } from "effector-react";
import {useEffect} from "react";
import {$content, contentFx} from "./state";

export const Content = () => {
  const content = useStore($content);

  useEffect(() => {
    contentFx({})
  })

  return <>{content}</>;
};
