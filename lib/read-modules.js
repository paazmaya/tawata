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
  getRepoPackage = require('./get-repo-package'),
  checkFilesProperty = require('./check-files-property');

const readModules = (list, options) => {

  const promises = list.map((item) => {
    const pkg = readPackage(item, options);
    if (!pkg) {
      return false;
    }
    const repoName = getReponame(pkg);
    console.log('repoName', repoName);
    if (!repoName) {
      return false;
    }

    // Is there files prop in the local copy?
    const hasFiles = checkFilesProperty(pkg, options);
    console.log(item, 'has files', hasFiles);

    // Is there files prop in the master branch?
    return getRepoPackage(repoName, options).then((files) => {
      console.log('files', files);
    });

    // Is there .npmignore file in the master branch?
    // return getRepoIgnore(repoName, options);
  });

  Promise.all(promises).then((data) => {
    console.log('All done?');
    console.log(data);
  });
};

module.exports = readModules;
