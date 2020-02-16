#!/bin/env node

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


const path = require('path');

const optionator = require('optionator');

const tawata = require('../index');

const PKG = require('../package.json');

const optsParser = optionator({
  prepend: `Usage: ${PKG.name} [options] <target path, defaults to './node_modules'>`,
  append: `Version ${PKG.version}`,
  options: [
    {
      option: 'help',
      alias: 'h',
      type: 'Boolean',
      description: 'Help and usage instructions'
    },
    {
      option: 'version',
      alias: 'V',
      type: 'Boolean',
      description: 'Version number',
      example: '-V'
    },
    {
      option: 'verbose',
      alias: 'v',
      type: 'Boolean',
      description: 'Verbose output, will give more information printed out'
    },
    {
      option: 'token',
      alias: 't',
      type: 'String',
      description: 'GitHub API personal authentication token'
    }
  ]
});


let opts;

try {
  opts = optsParser.parse(process.argv);
}
catch (error) {
  console.error(error.message);
  process.exit(1);
}

if (opts.version) {
  console.log(PKG.version);
  process.exit(0);
}

console.log(`${PKG.name} - ${PKG.description}`);

if (opts.help) {
  console.log(optsParser.generateHelp());
  process.exit(0);
}

// Does current working directory contain node_modules?
let dirpath = path.resolve('node_modules');
if (opts._.length === 1) {
  dirpath = path.resolve(opts._[0]);
}

const token = opts.token || process.env.GITHUB_TOKEN;

if (!token) {
  console.log('GitHub authentication token missing');
  console.log('Please set it via GITHUB_TOKEN environment variable or --token option');
  process.exit(1);
}

tawata(dirpath, {
  token: token,
  verbose: typeof opts.verbose === 'boolean' ?
    opts.verbose :
    false
});
