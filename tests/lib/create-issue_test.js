
const tape = require('tape'),
  createIssue = require('../../lib/create-issue');

tape('createIssue - Function takes two arguments', (test) => {
  test.plan(1);

  test.equal(createIssue.length, 2);
});
