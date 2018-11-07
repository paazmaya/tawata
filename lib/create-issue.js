/**
 * tawata (多和田)
 * https://github.com/paazmaya/tawata
 *
 * Check that the currently installed local Node.js modules have either `files` property in their `package.json`
 * or are using `.npmignone` in their source repository.
 *
 * In case neither exists, create an issue to the modules GitHub repository for adding such meta data.
 *
 * Copyright (c) Juga Paazmaya <paazmaya@yahoo.com> (https://paazmaya.fi)
 * Licensed under the MIT license
 */
'use strict';

const got = require('got');

const gotConfig = require('./got-config');

/**
 * Create an issue to the given repository.
 *
 * @param {string} repo person/name
 * @param {string} token The API token for GitHub, which should 40 characters long
 * @return {Promise} Promise which resolves with the API result
 * @see https://developer.github.com/v3/issues/#create-an-issue
 */
const createIssue = (repo, token) => {
  const url = `https://api.github.com/repos/${repo}/issues`;

  const data = {
    title: 'Reduce the size of the npm package by limiting the included files',
    body: `Looks like the \`files\` property (https://docs.npmjs.com/files/package.json#files) is not used in \`package.json\` to specify the included files, nor is the \`.npmignore\` file (https://docs.npmjs.com/misc/developers#keeping-files-out-of-your-package) is being used for blacklisting unwanted files, for the package published to npm.

Would you consider adding either the \`files\` property or the \`.npmignore\` file, so that the resulting package file would have smaller size?

The current size can be seen when executing the command \`npm pack\` (https://docs.npmjs.com/cli/pack).

> This issue was create via [tawata](https://github.com/paazmaya/tawata/)
`
  };

  const config = Object.assign(
    {},
    gotConfig(token),
    {
      method: 'POST',
      body: data
    }
  );

  return got(url, config)
    .then(res => {
      console.log('url', url);
      console.log(res.body);
    }).catch(error => {
      console.error(error.response.message);
      //=> 'Internal server error ...'
    });
};

module.exports = createIssue;
