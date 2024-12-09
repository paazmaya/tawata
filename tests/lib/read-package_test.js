import path from 'node:path';

import tape from 'tape';
import readPackage from '../../lib/read-package.js';

tape('readPackage - file does not exist', (test) => {
  test.plan(1);

  const input = 'hoplaa';

  const output = readPackage(input);

  test.notOk(output);
});

tape('readPackage - file does exist', (test) => {
  test.plan(1);

  const input = path.join(new URL('.', import.meta.url).pathname, '..', '..');

  const output = readPackage(input);

  test.equal(output.name, 'tawata');
});
