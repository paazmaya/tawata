# tawata (多和田)

> Check whitelist/blacklist of included files for each dependency
> and create issues when needed

[![dependencies Status](https://david-dm.org/paazmaya/tawata/status.svg)](https://david-dm.org/paazmaya/tawata)
[![CircleCI](https://circleci.com/gh/paazmaya/tawata.svg?style=svg)](https://circleci.com/gh/paazmaya/tawata)
[![codecov](https://codecov.io/gh/paazmaya/tawata/branch/master/graph/badge.svg)](https://codecov.io/gh/paazmaya/tawata)

This tool will get a list of all the [Node.js](https://nodejs.org/en/) dependencies under the current working directory, followed by these steps:

1. Which of those dependencies have [the `files` property defined in `package.json` file](https://docs.npmjs.com/files/package.json#files), in their GitHub repository?
1. Which of those that did not have the `files` property, have [the `.npmignore` file](https://docs.npmjs.com/misc/developers#keeping-files-out-of-your-package) existing in their GitHub repository?
1. For the remaining, create an issue to their GitHub repository for adding either the `files` property or `.npmignore`, so that the resulting package would become possibly smaller.

In order to see if the resulting package becomes any smaller, test it before by creating the package with [the command `npm pack`](https://docs.npmjs.com/cli/pack), then rename the resulting file so it would not be overwritten.
Then add the limitations and create the package again and see the file size difference.

## Background for the name

The name of the project is for honouring the legacy of a late master, Mr Tawata from the Ryukyu archipelago, who contributed to the martial arts that we today know as **karate** and **ryukyu kobujutsu**.

There are forms carrying his name, such as "Tawata no Passai" and ["Tawata no Sai"](https://www.youtube.com/watch?v=YAQlFJIj32E), which both have distinctive use of a stance called "nekoashidachi" and sweeping motion related to the given stance.

## Roadmap

The first version tested with real life usage turned out to be considered as spam #9,
and unnecessary noise. The first release version, namely `v0.1.0` asked the user for each individual issue to be created.

Further than just checking the existence of `files` property and `.npmignore` file, this tool should be able to check for things such as `license` property and `LICENSE*` file.

## Installation

Install globally as a command line tool, while expecting to have [Node.js](https://nodejs.org/en/) already installed, and hence the `npm` tool available:

```sh
[sudo] npm install --global tawata
```

Please note that the minimum supported version of [Node.js](https://nodejs.org/en/) is `10.13.0`, which is [the active Long Term Support (LTS) version](https://github.com/nodejs/Release#release-schedule).

GitHub API token is needed, and it should be made available either via the `--token` command line option,
or via an environment variable called `GITHUB_TOKEN`.

## Command line usage

The easiest way to get started with the command line interface, is to see the help output:

```sh
tawata --help
```

It provides the following output, which describes the possible usage options:

```sh
tawata - Check whitelist/blacklist of included files for each dependency and create issues when needed
Usage: tawata [options] <target path, defaults to './node_modules'>

  -h, --help          Help and usage instructions
  -V, --version       Version number
  -v, --verbose       Verbose output, will give more information printed out
  -t, --token String  GitHub API personal authentication token

Version 0.1.0
```

## Contributing

["A Beginner's Guide to Open Source: The Best Advice for Making your First Contribution"](http://www.erikaheidi.com/blog/a-beginners-guide-to-open-source-the-best-advice-for-making-your-first-contribution/).

[Also there is a blog post about "45 Github Issues Dos and Don’ts"](https://davidwalsh.name/45-github-issues-dos-donts).

Linting is done with [ESLint](http://eslint.org) and can be executed with `npm run lint`.
There should be no errors appearing after any JavaScript file changes.

Unit tests are written with [`tape`](https://github.com/substack/tape) and can be executed with `npm test`.
Code coverage is inspected with [`nyc`](https://github.com/istanbuljs/nyc) and
can be executed with `npm run coverage` after running `npm test`.
Please make sure it is over 90% at all times.

## Version history

* `v0.1.0` (2018-11-)
  - Use [`npm-shrinkwrap.json`](https://docs.npmjs.com/files/shrinkwrap.json) for locking the working set of 3rd party dependencies
  - Initial functionality which checks for existence of `files` property and `.npmignore` file

## License

Copyright (c) [Juga Paazmaya](https://paazmaya.fi) <paazmaya@yahoo.com>

Licensed under the [MIT license](LICENSE).
