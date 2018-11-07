
const tape = require('tape'),
  readModules = require('../../lib/read-modules');

tape('readModules - empty list', (test) => {

  test.plan(1);

  const input = [];

  const output = readModules(input, '');

  test.deepEqual(output, {});

});
