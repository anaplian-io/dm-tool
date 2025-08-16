import * as fs from 'node:fs';

const mcp = {
  'dm-tool': {
    command: 'node',
    args: [`${process.cwd()}/dist/index.js`],
  },
};

fs.writeFileSync('../../mcp.json', JSON.stringify(mcp, null, 2));
