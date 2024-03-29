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


/**
 * Configuration object for GitHub API connection with got.
 *
 * @param {string} token The API token for GitHub, which should 40 characters long
 * @returns {Object} Configuration object for got
 */
const gotConfig = (token) => ({
  json: true,
  headers: {
    accept: 'application/vnd.github.v3+json',
    authorization: `token ${token}`,
    'user-agent': 'https://github.com/paazmaya/tawata'
  },
  method: 'GET'
});

export default gotConfig;
