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

const readPackage = require('./read-package'),
  getReponame = require('./get-repo-name'),
  getRepoPackage = require('./get-repo-package');

/**
 * Read the package.json file from each module listed in the file system list,
 * then figure out the repository name at GitHub for it,
 * then fetch the current master version of the package.json file.
 *
 * @param {Array} list List of directories under node_modules, with their absolute paths
 * @returns {Promise} Promise that resolves with an Array of package.json Objects
 */
const readModules = (list, options) => {

  const promises = list.map((item) => {
    const pkg = readPackage(item);
    if (!pkg) {
      return false;
    }
    const repoName = getReponame(pkg);
    console.log('repoName', repoName);

    if (!repoName) {
      return false;
    }

    // Is there files prop in the master branch?
    return getRepoPackage(repoName, options.token);
  });

  Promise.all(promises).then((data) => {
    console.log('All done?');
    console.log(data);

    return data;
  });
};

module.exports = readModules;
