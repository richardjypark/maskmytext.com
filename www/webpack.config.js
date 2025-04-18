const CopyWebpackPlugin = require("copy-webpack-plugin");
const path = require("path");

module.exports = {
  entry: "./bootstrap.js",
  output: {
    path: path.resolve(__dirname, "dist"),
    filename: "bootstrap.js",
    publicPath: process.env.NODE_ENV === "production" ? "/mask-my-text/" : "/",
  },
  mode: process.env.NODE_ENV === "production" ? "production" : "development",
  plugins: [
    new CopyWebpackPlugin({
      patterns: [
        { from: "index.html" },
        { from: "manifest.json" },
        { from: "service-worker.js" },
        { from: "icons", to: "icons" },
        { from: "../pkg", to: "pkg" },
      ],
    }),
  ],
  experiments: {
    asyncWebAssembly: true,
  },
  module: {
    rules: [
      {
        test: /\.wasm$/,
        type: "webassembly/async",
      },
    ],
  },
  resolve: {
    extensions: [".js", ".wasm"],
    alias: {
      "mask-my-text": path.resolve(__dirname, "../pkg"),
    },
  },
  devServer: {
    static: {
      directory: path.join(__dirname, "dist"),
    },
    compress: true,
    port: 8080,
    historyApiFallback: true,
    hot: false,
    liveReload: false,
    client: false,
    webSocketServer: false,
  },
};
