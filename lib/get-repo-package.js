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


import got from 'got';

import gotConfig from './got-config.js';
import parseContent from './parse-content.js';
import parseJson from './parse-json.js';

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

  //console.log('got url', url);

  return got(url, config)
    .then((response) => {
      console.log(url, 'status:', response.statusCode);

      return response.body;
    })
    .then(parseContent)
    .then(parseJson)
    .catch((error) => {
      console.error('Error appears in getRepoPackage', error.response.statusCode);
      //console.error(error.response);
    });
};

export default getRepoPackage;
