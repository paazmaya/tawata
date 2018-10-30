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

/**
 * Create an issue to the given repository.
 *
 * @param {string} repo person/name
 * @return {Promise}
 * @see https://developer.github.com/v3/issues/#create-an-issue
 */
const createIssue = (repo, options) => {
  const url = `https://api.github.com/repos/${repo}/issues`;
  const data = {
    title: 'Limit the included files in the package published to npm',
    body: `Seems that neither 'files' property is used in 'package.json' to whitelist the included files, nor '.npmignore' file is being used for blacklisting included files.
    `
  };

  return got(url, {
    json: true,
    headers: {
      accept: 'application/vnd.github.v3+json',
      authorization: `token ${options.token}`,
      'user-agent': 'https://github.com/paazmaya/tawata'
    },
    method: 'POST',
    body: data
  }).then(res => {
    console.log('url', url);
    console.log(res.body);
  }).catch(error => {
    console.log(error.response);
    //=> 'Internal server error ...'
  });
};

module.exports = createIssue;
