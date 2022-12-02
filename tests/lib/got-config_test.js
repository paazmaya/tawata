
import tape from 'tape';
import gotConfig from '../../lib/got-config.js';

tape('gotConfig - token gets used', (test) => {
  test.plan(1);

  const input = 'hoplaa';

  const output = gotConfig(input);

  test.equal(output.headers.authorization, 'token hoplaa');
});
