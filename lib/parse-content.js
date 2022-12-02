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


/**
 * Parse content from got.
 *
 * @param {Object} body Response object from got
 * @return {string|Object} Parsed string, or the original input
 */
const parseContent = (body) => {
  if (body.content && body.encoding) {
    const data = Buffer.from(body.content, body.encoding);

    return data.toString('utf8');
  }

  return body;
};

export default parseContent;
