
import tape from 'tape';
import readModules from '../../lib/read-modules.js';

tape('readModules - empty list', (test) => {

  test.plan(1);

  const input = [];

  const output = readModules(input, '');

  test.equal(Object.keys(output).length, 0);

});
