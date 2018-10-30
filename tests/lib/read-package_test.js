
const tape = require('tape'),
  readPackage = require('../../lib/read-package');

tape('readPackage - files does not exist', (test) => {
  test.plan(1);

  const input = {
    hoplaa: ''
  };

  const output = readPackage(input);

  test.notOk(output);
});
