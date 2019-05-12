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

const readlineSync = require('readline-sync');

const readModules = require('./lib/read-modules'),
  readReponame = require('./lib/read-repo-name'),
  getRepoIgnore = require('./lib/get-repo-ignore'),
  createIssue = require('./lib/create-issue'),
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
 * Confirm creating an issue, based on the user input.
 *
 * @param {string} item person/name
 * @return {boolean} True when the issue should be created
 */
const confirmIssue = (item) => {
  const answer = readlineSync.question(`Create issue in GitHub for "${item}" y/N:`);

  return answer.match(/^y(es)?$/iu);
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

  // Which of those have the files property?
  // Which of those that did not have the files property, have .npmignore file?
  // For the remaining, create an issue.

  const createIssuesNeeded = [];

  return readModules(list, options).then((data) => {

    // Do something with the latest package.json from GitHub repository
    const checkIgnorePromises = data
      .filter((pkg) => {
        return !checkFilesProperty(pkg);
      })
      .map((pkg) => {
        return readReponame(pkg);
      })
      .filter((item) => item !== false)
      .map((item) => {
        return getRepoIgnore(item, options.token)
          .catch((error) => {
            console.error('Error appears with', item, error.response.statusCode);
            createIssuesNeeded.push(item);
          });
      });

    console.log('Should check how many for .npmignore file:', checkIgnorePromises.length);

    return Promise.all(checkIgnorePromises)
      .then((ignoreList) => {
        console.log('All ignores done?');

        return ignoreList;
      });
  }).then(() => {
    console.log('createIssuesNeeded.length', createIssuesNeeded.length);

    return createIssuesNeeded.map((item) => {
      if (confirmIssue(item)) {
        return createIssue(item, options.token);
      }

      return false;
    });
  }).then((issueList) => {
    return Promise.all(issueList);
  }).then(() => {
    console.log('Issues created!');
  });
};

module.exports = tawata;
