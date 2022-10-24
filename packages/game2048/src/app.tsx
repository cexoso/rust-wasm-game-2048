import React, { useRef } from "react";
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

export const GameApp = () => {
  const game = useGame();
  return (
    <GameContext.Provider value={game}>
      <GameUI />
    </GameContext.Provider>
  );
};
