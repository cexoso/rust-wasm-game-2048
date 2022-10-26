import React, { useRef, useEffect } from "react";
import { Game } from "core-wasm";
import { GameContext } from "./hooks/game";
import { GameUI } from "./components/game-ui";

// import { Board } from "./components/board";

function useConstant<T>(fn: () => T) {
  const ref = useRef<T>();
  if (ref.current === undefined) {
    ref.current = fn();
  }
  return ref.current;
}
function useGame() {
  return useConstant(() => {
    const game = new Game();
    game.init();
    return game;
  });
}

const useBindKeyBoard = (game: Game) => {
  useEffect(() => {
    const handleKeyPress = (event: KeyboardEvent) => {
      const { key } = event;
      if (key === "h") {
        game.left();
      } else if (key === "j") {
        game.down();
      } else if (key === "k") {
        game.up();
      } else if (key === "l") {
        game.right();
      }
    };
    window.addEventListener("keypress", handleKeyPress);
    return () => {
      window.removeEventListener("keypress", handleKeyPress);
    };
  }, [game]);
};

export const GameApp = () => {
  const game = useGame();
  useBindKeyBoard(game);
  return (
    <GameContext.Provider value={game}>
      <GameUI />
    </GameContext.Provider>
  );
};
