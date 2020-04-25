const path = require('path');

const spoilerRoot = './spoiler/';

module.exports = {
  src: spoilerRoot,
  schema: "./schema.graphql",
  artifactDirectory: path.join(spoilerRoot, "__generated__"),
  customScalars: {
    "Date": "string"
  }
};
