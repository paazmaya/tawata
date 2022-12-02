
import tape from 'tape';
import parseContent from '../../lib/parse-content.js';

tape('parseContent - returns self if no encoding', (test) => {
  test.plan(1);

  const input = {
    stuff: ''
  };

  const output = parseContent(input);

  test.deepEqual(output, input);
});

tape('parseContent - reads input as expected', (test) => {
  test.plan(1);

  const text = 'senkin tonttu';

  const input = {
    content: text,
    encoding: 'utf8'
  };

  const output = parseContent(input);

  test.equal(output, text);
});
