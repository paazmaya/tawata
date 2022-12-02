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


import fs from 'fs';
import path from 'path';

import parseJson from './parse-json.js';

/**
 * Read the package.json file from the given module directory.
 *
 * @param {string} dirpath Directory of the module
 * @return {Object|bool} Package information. False when not existing
 */
const readPackage = (dirpath) => {
  // It is always included in npm package, no matter what, hence should be there
  const filepath = path.join(dirpath, 'package.json');

  try {
    fs.accessSync(filepath);
  }
  catch (error) {
    return false;
  }

  const data = fs.readFileSync(filepath, 'utf8');

  return parseJson(data);
};

export default readPackage;
