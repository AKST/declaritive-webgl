const fs = require('fs')
const path = require("path");

const webpack = require("webpack");
const { CheckerPlugin } = require('awesome-typescript-loader');
const HtmlWebpackPlugin = require("html-webpack-plugin");
const ExtractTextPlugin = require("extract-text-webpack-plugin");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");

function createAliasObject(rootDirectory) {
  const srcDir = path.resolve(__dirname, rootDirectory);
  return fs.readdirSync(srcDir)
    .filter(f => fs.statSync(path.join(srcDir, f)).isDirectory())
    .map(f => ([f, path.join(srcDir, f)]))
    .reduce((o, [k, v]) => (o[k] = v, o), {});
}


module.exports = {
  devtool: 'source-map',
  watchOptions: {
    ignored: [
      'public',
      'node_modules',
    ],
  },
  entry: {
    main: './src/index.tsx'
  },
  output: {
    path: path.resolve(__dirname, "public"),
    filename: "[name].[chunkhash].chunk.js",
    chunkFilename: "[chunkhash].[id].js"
  },
  resolve: {
    extensions: ['.tsx', '.ts', '.js', '.jsx', '.wasm'],
    alias: {
      ...createAliasObject('src'),
      rust_runtime: path.resolve(__dirname, "runtime"),
      shaders: path.resolve(__dirname, "shaders"),
    },
  },
  module: {
    rules: [
      {
        test: /\.css$/,
        use: [
          { loader: 'style-loader' },
          {
            loader: 'css-loader',
            options: {
              modules: true,
            },
          },
        ],
      },
      {
        test: /\.tsx?$/,
        use: [
          {
            loader: 'awesome-typescript-loader',
            options: {
              useCache: true,
            },
          },
        ],
        exclude: /node_modules/,
      },
      {
        enforce: "pre",
        exclude: /node_modules/,
        test: /\.js$/,
        use: [
          // {
          //   loader: 'babel-loader',
          //   options: {
          //     presets: ['@babel/preset-env'],
          //     plugins: ["@babel/plugin-transform-typescript"],
          //   },
          // },
          "source-map-loader",
        ],
      },
      {
        test: /\.(frag|vert|glsl)$/,
        use: [
          {
            loader: 'raw-loader'
          }
        ]
      }
    ],
  },
  plugins: [
    new CheckerPlugin(),

    new HtmlWebpackPlugin(),

    new WasmPackPlugin({
      crateDirectory: path.resolve(__dirname, "runtime"),
    }),
  ]
}
