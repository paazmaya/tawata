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

const readModules = require('./lib/read-modules'),
  checkFilesProperty = require('./lib/check-files-property');

const DOT_INIT = /^\./u;

/**
 * Find module candidate directories under the given directory.
 *
 * @param {string} dirpath Directory under which to look for directories
 * @returns {Array} List of directories that contain a package.json file
 */
const findModules = (dirpath) => {
  return fs.readdirSync(dirpath)
    .filter((item) => !DOT_INIT.test(item))
    .map((item) => path.join(dirpath, item))
    .filter((item) => {
      try {
        const stat = fs.statSync(item);

        return stat.isDirectory();
      }
      catch (error) {
        return false;
      }
    })
    .filter((item) => {
      try {
        fs.accessSync(path.join(item, 'package.json'));

        return true;
      }
      catch (error) {
        return false;
      }
    });
};

/**
 * Do the magic.
 *
 * @param {string} dirpath Directory under which to look for directories that should each contain package.json
 * @param {Object} options Options
 * @param {string} options.token GitHub API token
 * @returns {bool|Promise} False when something goes wrong or a Promise
 */
const tawata = (dirpath, options) => {
  console.log(options);
  // options.token for GitHub API

  try {
    fs.accessSync(dirpath);
  }
  catch (error) {
    console.error(`Looks like "${dirpath}" cannot be accessed`);
    console.error(error.message);

    return false;
  }

  const list = findModules(dirpath);
  console.log(`Found ${list.length} directories under node_modules`);

  return readModules(list, options).then((data) => {
    // Do something with the latest package.json from GitHub repository
    data.forEach((pkg) => {
      // Is there files prop in the online copy?
      const hasFiles = checkFilesProperty(pkg, options);
      console.log(pkg.name, 'has files', hasFiles);
    });
  });
};

module.exports = tawata;
