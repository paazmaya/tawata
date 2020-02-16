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
 * Parse the assumed JSON data.
 *
 * @param {string} input String data, that should be valid JSON
 * @return {Object|false} Parsed data or false when failed
 */
const parseJson = (input) => {
  let data = {};

  try {
    data = JSON.parse(input);
  }
  catch (error) {
    console.error('Could not parse JSON input.', error.message);

    return false;
  }

  return data;
};

module.exports = parseJson;
