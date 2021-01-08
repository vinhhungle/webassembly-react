// const path = require("path");
// const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");

module.exports = function override(config, env) {
  const wasmExtensionRegExp = /\.wasm$/;

  config.resolve.extensions.push(".wasm");

  config.module.rules.forEach((rule) => {
    (rule.oneOf || []).forEach((oneOf) => {
      if (oneOf.loader && oneOf.loader.indexOf("file-loader") >= 0) {
        // make file-loader ignore WASM files
        oneOf.exclude.push(wasmExtensionRegExp);
      }
    });
  });

  /** No need WasmPackPlugin: Rust build failed for some reason. Should build manually. */
  // config.plugins.push(
  //   new WasmPackPlugin({
  //     // watchDirectories: [path.resolve(__dirname, "./src/rust-wasm/src")],
  //     crateDirectory: path.resolve(__dirname, "./src/rust-wasm"),
  //     args: "--log-level warn",
  //     outDir: "pkg",
  //   })
  // );

  return config;
};
