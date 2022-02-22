import React from "react";
import ReactDOM from "react-dom";
import "./index.css";
import App from "./App";
// import init, { NightRunner } from "@nightrunner/nightrunner_lib";
import init, { NightRunner } from "@nightrunner/nightrunner_lib";
console.log(init);
import data from "./data.json";

await init();

let nr: NightRunner = new NightRunner(JSON.stringify(data));

ReactDOM.render(
  <React.StrictMode>
    <App engine={nr} />
  </React.StrictMode>,
  document.getElementById("root")
);
