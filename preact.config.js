// I don't understand what is going on here.
// Based on code snippets from:
//   - https://stackoverflow.com/questions/45742982
//   - https://github.com/developit/preact-cli/issues/437

export default (config, env, helpers) => {
   config.output.publicPath = '/fhmp/';

  // use the public path in your app as 'process.env.PUBLIC_PATH'
  config.plugins.push(
    new helpers.webpack.DefinePlugin({
      'process.env.PUBLIC_PATH': JSON.stringify(config.output.publicPath || '/')
    })
  );
};
