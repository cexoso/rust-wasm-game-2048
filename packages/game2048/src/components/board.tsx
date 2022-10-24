import React from "react";
import { useBoard } from "../hooks/game";
import { cube, container, row } from "./board.css";

export const Board = () => {
  const board = useBoard();
  return (
    <div className={container}>
      {board.map((rows, rowIndex) => (
        <div className={row} key={rowIndex}>
          {rows.map((value, valueIndex) => (
            <div className={cube} key={valueIndex}>
              {value !== 0 ? value : ""}
            </div>
          ))}
        </div>
      ))}
    </div>
  );
};
