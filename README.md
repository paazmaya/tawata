# tawata (多和田)

> Check whitelist/blacklist of included files for each dependency
> and create issues when needed

Please note that the minimum supported version of [Node.js](https://nodejs.org/en/) is `8.11.1`, which is [the active Long Term Support (LTS) version](https://github.com/nodejs/Release#release-schedule).

## Background for the name

The name of the project is for honouring the legacy of a certain master from the Ryukyu archipelago who contributed to the martial arts that we today know as **karate** and **ryukyu kobujutsu**.

## Command line usage

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

## License

Copyright (c) [Juga Paazmaya](https://paazmaya.fi) <paazmaya@yahoo.com>

Licensed under the [MIT license](LICENSE).
