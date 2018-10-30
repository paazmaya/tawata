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

const gotConfig = require('./got-config'),
  parseContent = require('./parse-content'),
  parseJson = require('./parse-json');

/**
 * Get the package.json file from the repository.
 *
 * @param {string} repo person/name
 * @param {string} token GitHub API token
 * @return {Promise} Promise which resolves with package.json
 * @see https://developer.github.com/v3/repos/contents/#get-contents
 */
const getRepoPackage = (repo, token) => {
  const url = `https://api.github.com/repos/${repo}/contents/package.json`,
    config = gotConfig(token);

  console.log('got url', url);

  return got(url, config).then(response => {
    console.log(response.body);
    console.log(response.statusCode);

    return response.body;
  }).then(parseContent).then(data => {
    return parseJson(data);
  }).catch(error => {
    console.log(error.response);
    //=> 'Internal server error ...'
  });
};

module.exports = getRepoPackage;
