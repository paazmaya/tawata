
import tape from 'tape';
import createIssue from '../../lib/create-issue.js';

tape('createIssue - Function takes two arguments', (test) => {
  test.plan(1);

  test.equal(createIssue.length, 2);
});
