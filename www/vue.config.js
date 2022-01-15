const path = require("path");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");
const webpack = require("webpack");

module.exports = {
  publicPath: '/vpm/',
  productionSourceMap: false,
  pages: {
    index: "src/pages/index/main.js",
  },
  configureWebpack: {        
    devServer: {
      headers: {
        "Cross-Origin-Embedder-Policy": "require-corp",
        "Cross-Origin-Opener-Policy": "same-origin",
      },
      // mimeTypes: { 'application/wasm': ['wasm'] },
    },
  },
  chainWebpack: (config) => {
    // rust wasm bindgen https://github.com/rustwasm/wasm-bindgen
    config
      .plugin("wasm-pack")
      .use(WasmPackPlugin)
      .init(
        (Plugin) =>
          new Plugin({
            crateDirectory: path.resolve(__dirname, "../rust/wasm"),
            outDir: path.resolve(__dirname, "./src/pkg"),
            // forceMode: "development",
            forceMode: "production",
	    // extraArgs: '--target web'
	    // extraArgs: '--target no-modules'
          })
      )
      .end()
      //  needed for Edge browser https://rustwasm.github.io/docs/wasm-bindgen/examples/hello-world.html
      .plugin("text-encoder")
      .use(webpack.ProvidePlugin)
      .init(
        (Plugin) =>
          new Plugin({
            TextDecoder: ["text-encoding", "TextDecoder"],
            TextEncoder: ["text-encoding", "TextEncoder"],
          })
      )
      .end();
    config.module.rule("js").exclude.add(/\.worker\.js$/);
    config.module
      .rule("worker")
      .test(/\.worker\.js$/)
      .use("worker-loader")
      .loader("worker-loader")
      .end();

    config.module
      .rule("loader-import-meta")
      .test(/index.js$/)
      .use('@open-wc/webpack-import-meta-loader')
      .loader('@open-wc/webpack-import-meta-loader')
      .end();
  },
};
