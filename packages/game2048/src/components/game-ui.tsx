import React from "react";
import { container } from "./game-ui.css";
import { Board } from "./board";

export const GameUI = () => {
  return (
    <div className={container}>
      <Board />
    </div>
  );
};
