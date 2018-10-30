
const tape = require('tape'),
  parseJson = require('../../lib/parse-json');

tape('parseJson - files does not exist', (test) => {
  test.plan(1);

  const input = {
    hoplaa: ''
  };

  const output = parseJson(input);

  test.notOk(output);
});
