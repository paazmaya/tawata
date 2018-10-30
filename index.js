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

const parseContent = (body) => {
  if (body.content && body.encoding) {
    const data = Buffer.from(body.content, body.encoding);

    return data.toString('utf8');
  }
  return body;
};

/**
 * Parse the assumed JSON data.
 *
 * @param {string} input String data, that should be valid JSON
 * @return {Object} Parsed data
 */
const parseJson = (input) => {
  let data = {};

  try {
    data = JSON.parse(input);
  }
  catch (error) {
    console.error('Could not parse JSON input');
    console.error(error);
  }
  return data;
};

/**
 * Read the package.json file from the given module directory.
 *
 * @param  {string} dirpath Directory of the module
 * @return {Object} Package information
 */
const readPackage = (dirpath, options) => {
  // It is always included in npm package, no matter what, hence should be there
  const filepath = path.join(dirpath, 'package.json');
  console.log('filepath', filepath);

  const data = fs.readFileSync(filepath, 'utf8');
  return parseJson(data);
}

/**
 * Does the given module have files property.
 *
 * @param  {string} dirpath Directory of the module
 * @return {bool|Number} False or a number of files listed in the files property
 * @see https://docs.npmjs.com/files/package.json#files
 */
const checkFilesProperty = (pkg, options) => {
  /*
  return new Promise(function(resolve, reject) {

  });
  */

  if (Object.prototype.hasOwnProperty.call(pkg, 'files') && Array.isArray(pkg.files)) {
    console.log('pkg.files', pkg.files);
    return pkg.files;
  }
  return false;
};

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
  repoName = repoName.replace(/\.git$/, '').replace(/.+github\.com\//, '');
  return repoName;
};

const readModules = (dirpath,options) => {
  const list = fs.readdirSync(dirpath)
    .map((item) => path.join(dirpath, item))
    .filter((item) => {
      const stat = fs.statSync(item);
      return stat.isDirectory();
    });
  console.log(`Found ${list.length} directories under node_modules`);

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

module.exports = function (options) {
  console.log(options);
  // options.token for GitHub API
  // Does current working directory contain node_modules?
  const dirpath = path.resolve('node_modules');

  try {
    fs.accessSync(dirpath);
    readModules(dirpath, options);
  } catch(error) {
    console.error(`Looks like "${dirpath}" cannot be accessed`);
    console.error(error.message);
  }
};
