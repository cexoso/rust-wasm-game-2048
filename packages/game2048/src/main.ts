import "./style.css";
import { Adder, Game } from "core-wasm";

const adder = new Adder();
const game = new Game();
adder.add(123);
console.log(adder.add(432));
game.subscript((event: any) => {
  console.log("debugger 🐛 event", event);
});
game.init();
const app = document.getElementById("app")!;

app.innerText = `${adder.hello()}, this message from wasm`;
