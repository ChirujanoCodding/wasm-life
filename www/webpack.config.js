const CopyWebpackPlugin = require("copy-webpack-plugin");
const path = require('path');

const crypto = require('crypto')
const cryptoOrigCreateHash = crypto.createHash
crypto.createHash = algorithm => cryptoOrigCreateHash(algorithm === 'md4' ? 'sha256' : algorithm)

module.exports = {
  entry: "./bootstrap.js",
  output: {
    path: path.resolve(__dirname, "dist"),
    filename: "bootstrap.js",
  },
  mode: "development",
  plugins: [
    new CopyWebpackPlugin(['index.html'])
  ],
};
