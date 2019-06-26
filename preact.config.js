const GitRevisionPlugin = require('git-revision-webpack-plugin');

export default (config, env, helpers) => {
  // I don't understand what is going on here.
  // Based on the code snippets from:
  //   - https://stackoverflow.com/questions/45742982
  //   - https://github.com/developit/preact-cli/issues/437
  config.output.publicPath = env.production ? '/fhmp/' : '/';

  const gitRevisionPlugin = new GitRevisionPlugin();
  config.plugins.push(
    new helpers.webpack.DefinePlugin({
      'process.env.PUBLIC_PATH': config.output.publicPath,
      'COMMITHASH': JSON.stringify(gitRevisionPlugin.commithash())
    })
  );
};
