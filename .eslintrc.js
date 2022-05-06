module.exports = {
  extends: ['paazmaya', 'plugin:node/recommended'],
  parserOptions: {
    ecmaVersion: 'latest'
  },
  rules: {
    'max-lines-per-function': [1, 24],
    'no-console': 0,
    'no-process-env': 0,
    'no-process-exit': 0
  }
};
