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
 * Check if the given module has .npmignore file in their repository master branch.
 *
 * @param  {string} dirpath Directory of the module
 * @return {bool|Number} False or a number of files listed in the .npmignore file
 * @see https://docs.npmjs.com/files/package.json#repository
 */
const getReponame = (pkg) => {

  if (!Object.prototype.hasOwnProperty.call(pkg, 'repository')) {
    return false;
  }

  let repoName = '';
  if (typeof pkg.repository === 'string') {
    repoName = pkg.repository;
  }
  else if (Object.prototype.hasOwnProperty.call(pkg.repository, 'url')) {
    repoName = pkg.repository.url;
  }
  else {
    return false;
  }
  // Take the last word/word part, such as paazmaya/tawata
  repoName = repoName.replace(/\.git$/u, '').replace(/.+github\.com\//u, '');

  return repoName;
};

module.exports = getReponame;
