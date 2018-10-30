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
'use strict';

const path = require('path');

const tawata = require('../index');

const options = {
  token: process.env.GITHUB_TOKEN || ''
};

// Does current working directory contain node_modules?
const dirpath = path.resolve('node_modules');

tawata(dirpath, options);
