import "./style.css";
import { Adder } from "core-wasm";

const adder = new Adder();
adder.add(123);
console.log(adder.add(432));
const app = document.getElementById("app")!;

app.innerText = `${adder.hello()}, this message from wasm`;
