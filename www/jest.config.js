module.exports = {
  preset: '@vue/cli-plugin-unit-jest',
  collectCoverage: true,
  collectCoverageFrom: ['**/src/**/*.{js,vue}', '!**/src/**/*.(test|spec).js', '!**/node_modules/**', '!**/pkg/**' ],
  testMatch: [ '**/src/**/*.(test|spec).js?(x)', '**/tests/**/*.(test|spec).js', '!**/node_modules/**' ],
  coverageProvider: 'v8',
}
