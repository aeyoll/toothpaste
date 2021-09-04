const webpack = require('webpack');
const { merge } = require('webpack-merge');
const CopyPlugin = require('copy-webpack-plugin');
const MiniCssExtractPlugin = require('mini-css-extract-plugin');

const common = require('./webpack.common.js');

module.exports = merge(common, {
  mode: 'development',
  devtool: false,
//   module: {
//     rules: [
//       {
//         test: /\.(png|jpe?g|gif|svg|ico)$/,
//         type: 'asset/resource',
//         generator: {
//           filename: 'images/[name][ext][query]',
//         },
//       },
//       {
//         test: /\.(ttf|eot|woff2?)$/,
//         type: 'asset/resource',
//         generator: {
//           filename: 'fonts/[name][ext][query]',
//         },
//       },
//     ],
//   },
  plugins: [
    new webpack.SourceMapDevToolPlugin({
      filename: '[file].map',
    }),
    new MiniCssExtractPlugin({
      filename: 'styles/[name].css',
      chunkFilename: 'styles/[id].css',
    }),
    // new CopyPlugin({
    //   patterns: [
    //     {
    //       from: 'images',
    //       to: 'images/[path][name].[ext]',
    //     },
    //     {
    //       from: 'fonts',
    //       to: 'fonts/[path][name].[ext]',
    //     },
    //   ],
    // }),
  ],
});
