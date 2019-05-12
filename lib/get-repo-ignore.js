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
 * Get the .npmignore file from the repository.
 *
 * @param {string} repo person/name
 * @param {string} token The API token for GitHub, which should 40 characters long
 * @return {Promise}
 * @see https://developer.github.com/v3/repos/contents/#get-contents
 */
const getRepoIgnore = (repo, token) => {
  const url = `https://api.github.com/repos/${repo}/contents/.npmignore`,
    config = gotConfig(token);

  // Do not catch here, since it should be caught outside.
  return got(url, config)
    .then((response) => {
      console.log(url, 'status:', response.statusCode);

      return response;
    });
};

module.exports = getRepoIgnore;
