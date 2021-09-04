module.exports = ({ file, options }) => {// eslint-disable-line no-unused-vars
  return {
    parser: 'postcss-safe-parser',
    plugins: {
      autoprefixer: true,
    },
  };
};
