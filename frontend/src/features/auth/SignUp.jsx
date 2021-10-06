import styles from "./Auth.module.css";
import { useState } from "react";
import React from "react";

const SignUp = () => {
  const [nickname, setNickname] = useState("");
  const [name, setName] = useState("");
  const [password, setPassword] = useState("");
  const [confirmation, setConfirmation] = useState("");

  const handleSubmit = (e) => {
    e.preventDefault();
  };

  return (
    <div className={styles.container}>
      <div className={styles.card}>
        <h1>Welcome, Sempai :)</h1>
        <h2>First time? Let's sign up</h2>
        <form onSubmit={handleSubmit}>
          <input
            name="nickname"
            placeholder="Nickname"
            className={styles.textField}
            type="text"
            value={nickname}
            onChange={(e) => {
              setNickname(e.target.value);
            }}
          />
          <br />
          <input
            name="name"
            placeholder="How to call you, Sempai?"
            className={styles.textField}
            type="text"
            value={name}
            onChange={(e) => {
              setName(e.target.value);
            }}
          />
          <br />
          <input
            name="password"
            placeholder="Password"
            className={styles.textField}
            type="password"
            value={password}
            onChange={(e) => {
              setPassword(e.target.value);
            }}
          />
          <br />
          <input
            name="confirmation"
            placeholder="Confirmation"
            className={styles.textField}
            type="password"
            value={confirmation}
            onChange={(e) => {
              setConfirmation(e.target.value);
            }}
          />
          <br />
          <input className={styles.button} type="submit" value="Sign Up" />
        </form>
      </div>
    </div>
  );
};

export default SignUp;
