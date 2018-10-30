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

const readModules = require('./lib/read-modules');

const DOT_INIT = /^\./u;

module.exports = function (dirpath, options) {
  console.log(options);
  // options.token for GitHub API

  const list = fs.readdirSync(dirpath)
    .filter((item) => !DOT_INIT.test(item))
    .map((item) => path.join(dirpath, item))
    .filter((item) => {
      const stat = fs.statSync(item);

      return stat.isDirectory();
    });
  console.log(`Found ${list.length} directories under node_modules`);

  try {
    fs.accessSync(dirpath);
    readModules(list, options);
  }
  catch (error) {
    console.error(`Looks like "${dirpath}" cannot be accessed`);
    console.error(error.message);
  }
};
