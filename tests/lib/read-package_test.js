
const tape = require('tape'),
  readPackage = require('../../lib/read-package');

tape('readPackage - file does not exist', (test) => {
  test.plan(1);

  const input = 'hoplaa';

  const output = readPackage(input);

  test.notOk(output);
});

tape('readPackage - file does exist', (test) => {
  test.plan(1);

  const input = '../../';

  const output = readPackage(input);

  test.equal(output.name, 'tawata');
});
