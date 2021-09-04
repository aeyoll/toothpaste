'use strict'; // eslint-disable-line

const webpack = require('webpack');
const path = require('path');
const glob = require('glob');
const { CleanWebpackPlugin } = require('clean-webpack-plugin');
const MiniCssExtractPlugin = require('mini-css-extract-plugin');
const StyleLintPlugin = require('stylelint-webpack-plugin');
const ESLintPlugin = require('eslint-webpack-plugin');
const { WebpackManifestPlugin } = require('webpack-manifest-plugin');
const RemoveEmptyScriptsPlugin = require('webpack-remove-empty-scripts');
const PurgecssPlugin = require('purgecss-webpack-plugin');

const rootPath = process.cwd();

const assetPath = path.join(rootPath, 'assets');
const distPath = path.join(rootPath, 'static');
const publicPath = '/static/';

function collectSafelist() {
  return {
    standard: ['pre'],
    deep: [],
    greedy: [],
  };
}

module.exports = {
  entry: './assets/index.js',

  output: {
    path: distPath,
    filename: 'scripts/[name].js',
    publicPath: publicPath,
  },

  optimization: {
    usedExports: true,
    splitChunks: {
      cacheGroups: {
        commons: {
          test: /[\\/]node_modules[\\/]/,
          name: 'vendors',
          chunks: 'all',
        },
      },
    },
  },

  module: {
    rules: [
      {
        test: /\.js$/,
        exclude: /(node_modules)/,
        use: [
          { loader: 'swc-loader' },
        ],
      },
      {
        test: /\.scss$/,
        include: assetPath,
        use: [
          MiniCssExtractPlugin.loader,
          { loader: 'css-loader', options: { sourceMap: true } },
          { loader: 'postcss-loader', options: { sourceMap: true } },
          { loader: 'resolve-url-loader', options: { sourceMap: true } },
          { loader: 'sass-loader', options: { sourceMap: true } },
        ],
      },
    ],
  },

  plugins: [
    new CleanWebpackPlugin(),
    new ESLintPlugin(),
    new RemoveEmptyScriptsPlugin(),
    new PurgecssPlugin({
      paths: glob.sync('templates/**/*',  { nodir: true }),
      safelist: collectSafelist,
    }),
    new WebpackManifestPlugin({
      fileName: 'assets-manifest.json',
      publicPath: '',
    }),
    new StyleLintPlugin({
      failOnError: false,
      syntax: 'scss',
    }),
  ],

};
