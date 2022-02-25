import React from "react";
import ReactDOM from "react-dom";
import "./index.css";
import App from "./App";
import { NightRunner } from "@nightrunner/nightrunner_lib";
import data from "./data.json";

let engine: NightRunner = new NightRunner(JSON.stringify(data));

ReactDOM.render(
  <React.StrictMode>
    <App engine={engine} />
  </React.StrictMode>,
  document.getElementById("root")
);
