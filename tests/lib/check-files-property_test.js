
import tape from 'tape';
import checkFilesProperty from '../../lib/check-files-property.js';

tape('checkFilesProperty - files does not exist', (test) => {
  test.plan(1);

  const input = {
    hoplaa: ''
  };

  const output = checkFilesProperty(input);

  test.notOk(output);
});

tape('checkFilesProperty - files is not an array', (test) => {
  test.plan(1);

  const input = {
    files: ''
  };

  const output = checkFilesProperty(input);

  test.notOk(output);
});

tape('checkFilesProperty - null', (test) => {
  test.plan(1);

  const input = null;

  const output = checkFilesProperty(input);

  test.notOk(output);
});

tape('checkFilesProperty - files is an array with zero', (test) => {
  test.plan(1);

  const input = {
    files: []
  };

  const output = checkFilesProperty(input);

  test.equal(output, 0);
});

tape('checkFilesProperty - files is an array with five', (test) => {
  test.plan(1);

  const input = {
    files: [
      '1',
      '2',
      3,
      4,
      '5'
    ]
  };

  const output = checkFilesProperty(input);

  test.equal(output, 5);
});
