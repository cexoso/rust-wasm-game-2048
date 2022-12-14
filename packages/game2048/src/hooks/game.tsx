import { useContext, createContext, useState, useEffect } from "react";
import type { Game } from "core-wasm";

export type IBoardValue = [
  [number, number, number, number],
  [number, number, number, number],
  [number, number, number, number],
  [number, number, number, number]
];

export const GameContext = createContext<Game>(null as unknown as Game);

export const useGame = () => useContext(GameContext);

export const useBoard = () => {
  const game = useGame();
  const [board, setBoard] = useState<IBoardValue>(
    game.get_checkerboard_js_state()
  );
  useEffect(() => {
    const onBoardChange = (event: IBoardValue) => {
      setBoard(event);
    };
    game.subscript_board(onBoardChange);
    return () => {
      game.unsubscript_board(onBoardChange);
    };
  });
  return board;
};
