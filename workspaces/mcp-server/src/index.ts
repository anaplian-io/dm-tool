import { McpServer } from '@modelcontextprotocol/sdk/server/mcp.js';
import { StdioServerTransport } from '@modelcontextprotocol/sdk/server/stdio.js';
import { z } from 'zod';

(async () => {
  const server = new McpServer({
    name: 'dm-tool',
    version: '0.1.0',
  });

  server.registerTool(
    'roll',
    {
      title: 'Roll RPG Dice',
      description:
        'Rolls standard Dungeons and Dragons dice using a dice roll expression (e.g. 2d10+4d6+3)',
      inputSchema: { diceExpression: z.string() },
    },
    async ({ diceExpression }) => {
      let isError = false;
      const response = await fetch(
        `http://localhost:8080/v1/dice/roll/${diceExpression}`,
      )
        .then((response) => response.text())
        .catch((e) => {
          isError = true;
          return String(e);
        });
      return {
        isError,
        content: [
          {
            type: 'text',
            text: response,
          },
        ],
      };
    },
  );

  const transport = new StdioServerTransport();
  await server.connect(transport);
})();
