import React, { useEffect, useState } from "react";

import logo from "./logo.svg";
import "./App.css";

function fibonacci(n) {
  let a = 0;
  let b = 1;

  while (n >= 1) {
    let tmp = b;
    b += a;
    a = tmp;
    --n;
  }

  return a;
}

function bm_fibonacci(n, m) {
  for (var i = 0; i < m; i++) {
    fibonacci(n);
  }
}

const n = 40;
const m = 1000000;

function App() {
  const [state, setState] = useState("");

  useEffect(() => {
    import("rust-wasm").then((wasm) => {
      console.log({ wasm });

      console.log(wasm.add(99999999999, 99999999999));

      console.time("wasm_render_md");
      let markdown_input =
        "Hello world, this is a ~~complicated~~ *very simple* example.";
      let html_output = wasm.render_md(markdown_input);
      setState(html_output);
      console.log({ markdown_input, html_output });
      console.timeEnd("wasm_render_md");

      console.time("wasm_fibonacci");
      let r = wasm.fibonacci(n);
      console.log(r);
      wasm.bm_fibonacci(n, m);
      console.timeEnd("wasm_fibonacci");

      console.time("js_fibonacci");
      console.log(fibonacci(n));
      bm_fibonacci(n, m);
      console.timeEnd("js_fibonacci");

      let data = wasm.get_json();
      console.log("get_json", { data });

      console.log(
        "send_json",
        wasm.send_json({ ...data, field2: [...data.field2, [5, 6]] })
      );
    });
  }, []);

  return (
    <div className="App">
      <header className="App-header">
        <img src={logo} className="App-logo" alt="logo" />
        <p>Check dev console.</p>
        <div
          dangerouslySetInnerHTML={{
            __html: state,
          }}
        ></div>
        <a
          className="App-link"
          href="https://reactjs.org"
          target="_blank"
          rel="noopener noreferrer"
        >
          Learn React
        </a>
      </header>
    </div>
  );
}

export default App;
