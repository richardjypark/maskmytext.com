const CopyWebpackPlugin = require("copy-webpack-plugin");
const path = require("path");

// Get version from environment or generate timestamp-based version
const SW_VERSION = process.env.GITHUB_SHA
  ? `mask-my-text-${process.env.GITHUB_SHA.substring(0, 8)}`
  : `mask-my-text-${new Date()
      .toISOString()
      .replace(/[^0-9]/g, "")
      .slice(0, 14)}`;

module.exports = {
  entry: {
    bootstrap: "./bootstrap.js",
    "service-worker": "./service-worker.js",
  },
  output: {
    path: path.resolve(__dirname, "dist"),
    filename: "[name].js",
    publicPath: "./",
  },
  mode: process.env.NODE_ENV === "production" ? "production" : "development",
  plugins: [
    new CopyWebpackPlugin({
      patterns: [
        { from: "index.html" },
        { from: "manifest.json" },
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
      {
        test: /service-worker\.js$/,
        use: [
          {
            loader: "string-replace-loader",
            options: {
              search: 'const CACHE_NAME = ".*?"',
              replace: `const CACHE_NAME = "${SW_VERSION}"`,
              flags: "g",
            },
          },
        ],
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
