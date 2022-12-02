
import tape from 'tape';
import parseJson from '../../lib/parse-json.js';

tape('parseJson - not json input', (test) => {
  test.plan(1);

  const input = 'not-here';

  const output = parseJson(input);

  test.notOk(output);
});

tape('parseJson - json input', (test) => {
  test.plan(1);

  const input = '{"hello": "there"}';

  const output = parseJson(input);

  test.equal(output.hello, 'there');
});

tape('parseJson - null', (test) => {
  test.plan(1);

  const input = null;

  const output = parseJson(input);

  test.notOk(output);
});
