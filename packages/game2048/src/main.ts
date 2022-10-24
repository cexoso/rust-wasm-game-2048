import { createRoot } from "react-dom/client";
import React from "react";
import { GameApp } from "./app";

const app = document.getElementById("app")!;
const root = createRoot(app);
root.render(React.createElement(GameApp));
