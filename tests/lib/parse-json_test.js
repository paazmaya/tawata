
const tape = require('tape'),
  parseJson = require('../../lib/parse-json');

tape('parseJson - file does not exist', (test) => {
  test.plan(1);

  const input = 'not-here';

  const output = parseJson(input);

  test.notOk(output);
});
