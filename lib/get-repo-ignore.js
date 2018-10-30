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
 * Get the .npmignore file from the repository.
 *
 * @param {string} repo person/name
 * @return {Promise}
 * @see https://developer.github.com/v3/repos/contents/#get-contents
 */
const getRepoIgnore = (repo, options) => {
  const url = `https://api.github.com/repos/${repo}/contents/.npmignore`;

  return got(url, {
    json: true,
    headers: {
      'accept': 'application/vnd.github.v3+json',
      'authorization': `token ${options.token}`,
      'user-agent': 'https://github.com/paazmaya/tawata'
    },
    method: 'GET'
  }).then(response => {
    console.log('url', url);
    console.log(response.statusCode);
    console.log(Objec.keys(response));
  }).catch(error => {
    console.log(error.response.statusCode);
    console.log(Objec.keys(error.response));
    console.log(error.response);
    //=> 'Internal server error ...'
  });
};

module.exports = getRepoIgnore;
