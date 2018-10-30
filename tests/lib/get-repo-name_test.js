
const tape = require('tape'),
  getReponame = require('../../lib/get-repo-name');

tape('getReponame - files does not exist', (test) => {
  test.plan(1);

  const input = {
    hoplaa: ''
  };

  const output = getReponame(input);

  test.notOk(output);
});
