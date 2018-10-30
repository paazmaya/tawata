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

const fs = require('fs'),
  path = require('path');

const parseJson = require('./parse-json');

/**
 * Read the package.json file from the given module directory.
 *
 * @param  {string} dirpath Directory of the module
 * @return {Object} Package information
 */
const readPackage = (dirpath) => {
  // It is always included in npm package, no matter what, hence should be there
  const filepath = path.join(dirpath, 'package.json');

  const data = fs.readFileSync(filepath, 'utf8');

  return parseJson(data);
};

module.exports = readPackage;
