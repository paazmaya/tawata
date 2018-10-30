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

/**
 * Does the given module have files property.
 *
 * @param  {Object} pkg Package meta information from package.json
 * @return {bool|Number} False or a number of files listed in the files property
 * @see https://docs.npmjs.com/files/package.json#files
 */
const checkFilesProperty = (pkg) => {
  if (Reflect.has(pkg, 'files') && Array.isArray(pkg.files)) {

    return pkg.files.length;
  }

  return false;
};

module.exports = checkFilesProperty;
