
const tape = require('tape'),
  readModules = require('../../lib/read-modules');

tape('readModules - files does not exist', (test) => {
  test.plan(1);

  const input = {
    hoplaa: ''
  };

  const output = readModules(input);

  test.notOk(output);
});
