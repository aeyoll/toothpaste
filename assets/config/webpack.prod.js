const { merge } = require('webpack-merge');
const { default: ImageminPlugin } = require('imagemin-webpack-plugin');
const MiniCssExtractPlugin = require('mini-css-extract-plugin');
const CopyPlugin = require('copy-webpack-plugin');

const common = require('./webpack.common.js');

module.exports = merge(common, {
  mode: 'production',
  devtool: false,
  output: {
    filename: 'scripts/[name].[contenthash:8].js',
  },
//   module: {
//     rules: [
//       {
//         test: /\.(png|jpe?g|gif|svg|ico)$/,
//         type: 'asset/resource',
//         generator: {
//           filename: 'images/[name].[contenthash:8][ext][query]',
//         },
//       },
//       {
//         test: /\.(ttf|eot|woff2?)$/,
//         type: 'asset/resource',
//         generator: {
//           filename: 'fonts/[name].[contenthash:8][ext][query]',
//         },
//       },
//     ],
//   },
  plugins: [
    // new ImageminPlugin({
    //   optipng: { optimizationLevel: 7 },
    //   gifsicle: { optimizationLevel: 3 },
    //   pngquant: { quality: '65-90', speed: 4 },
    //   svgo: { removeUnknownsAndDefaults: false, cleanupIDs: false },
    //   plugins: [imageminMozjpeg({ quality: 75 })],
    // }),
    new MiniCssExtractPlugin({
      filename: 'styles/[name].[contenthash:8].css',
      chunkFilename: 'styles/[id].css',
    }),
    // new CopyPlugin({
    //   patterns: [
    //     {
    //       from: 'images',
    //       to: 'images/[path][name].[contenthash:8][ext]',
    //     },
    //     {
    //       from: 'fonts',
    //       to: 'fonts/[path][name].[contenthash:8][ext]',
    //     },
    //   ],
    // }),
  ],
});
