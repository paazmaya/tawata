
import tape from 'tape';
import readReponame from '../../lib/read-repo-name.js';

tape('getReponame - git url', (test) => {

  test.plan(1);

  const input = {
    repository: 'git@github.com:runk/node-chardet'
  };

  const output = readReponame(input);

  test.equal(output, 'runk/node-chardet');
});

tape('getReponame - https url', (test) => {

  test.plan(1);

  const input = {
    repository: {
      type: 'git',
      url: 'git+https://github.com/acornjs/acorn.git'
    }
  };

  const output = readReponame(input);

  test.equal(output, 'acornjs/acorn');
});

tape('getReponame - null', (test) => {

  test.plan(1);

  const input = null;

  const output = readReponame(input);

  test.ok(output === false);
});
