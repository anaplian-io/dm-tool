import { McpServer } from '@modelcontextprotocol/sdk/server/mcp.js';
import { StdioServerTransport } from '@modelcontextprotocol/sdk/server/stdio.js';
import { z } from 'zod';
import { invokeDmToolApi } from './invoke-dm-tool-api';

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
      annotations: {
        idempotentHint: true,
        readOnlyHint: true,
        destructiveHint: false,
        openWorldHint: false,
      },
    },
    async ({ diceExpression }) =>
      invokeDmToolApi(`/v1/dice/roll/${diceExpression}`),
  );

  server.registerTool(
    'list_dice',
    {
      title: 'List Dice',
      description:
        'Lists the available dice types that will be accepted by the server.',
      annotations: {
        idempotentHint: true,
        readOnlyHint: true,
        destructiveHint: false,
        openWorldHint: false,
      },
    },
    () => invokeDmToolApi('/v1/dice/list'),
  );

  server.registerTool(
    'search_monsters',
    {
      title: 'Search Monsters',
      description: 'Searches by keyword for available monsters.',
      inputSchema: { query: z.string() },
      annotations: {
        idempotentHint: true,
        readOnlyHint: true,
        destructiveHint: false,
        openWorldHint: false,
      },
    },
    async ({ query }) => invokeDmToolApi(`/v1/monsters?query=${query}`),
  );

  server.registerTool(
    'get_monster',
    {
      title: 'Get Monster',
      description: `Retrieves a specific monster's stats by name.`,
      inputSchema: { name: z.string() },
      annotations: {
        idempotentHint: true,
        readOnlyHint: true,
        destructiveHint: false,
        openWorldHint: false,
      },
    },
    async ({ name }) => invokeDmToolApi(`/v1/monsters/${name}`),
  );

  server.registerTool(
    'roll_monster_save',
    {
      title: 'Saving Monster Saving Throw',
      description: `Rolls a saving throw for a monster based on their stat block. Optionally can be rolled with advantage or disadvantage`,
      inputSchema: {
        name: z.string(),
        stat: z.string(),
        advantage: z.boolean().optional(),
        disadvantage: z.boolean().optional(),
      },
      annotations: {
        idempotentHint: true,
        readOnlyHint: true,
        destructiveHint: false,
        openWorldHint: false,
      },
    },
    async ({ name, stat, advantage, disadvantage }) => {
      if (advantage) {
        return invokeDmToolApi(
          `/v1/monsters/${name}/roll/throw/${stat}?advantage`,
        );
      }
      if (disadvantage) {
        return invokeDmToolApi(
          `/v1/monsters/${name}/roll/throw/${stat}?disadvantage`,
        );
      }
      return invokeDmToolApi(`/v1/monsters/${name}/roll/throw/${stat}`);
    },
  );

  server.registerTool(
    'roll_monster_skill',
    {
      title: 'Roll Monster Skill',
      description: `Rolls a monster's skill check. Optionally can be rolled with advantage or disadvantage`,
      inputSchema: {
        name: z.string(),
        skill: z.string(),
        advantage: z.boolean().optional(),
        disadvantage: z.boolean().optional(),
      },
      annotations: {
        idempotentHint: true,
        readOnlyHint: true,
        destructiveHint: false,
        openWorldHint: false,
      },
    },
    async ({ name, skill, advantage, disadvantage }) => {
      if (advantage) {
        return invokeDmToolApi(
          `/v1/monsters/${name}/roll/skill/${skill}?advantage`,
        );
      }
      if (disadvantage) {
        return invokeDmToolApi(
          `/v1/monsters/${name}/roll/skill/${skill}?disadvantage`,
        );
      }
      return invokeDmToolApi(`/v1/monsters/${name}/roll/skill/${skill}`);
    },
  );

  server.registerTool(
    'roll_monster_stat',
    {
      title: 'Roll Monster Stat',
      description: `Rolls a monster's stat check. Optionally can be rolled with advantage or disadvantage`,
      inputSchema: {
        name: z.string(),
        stat: z.string(),
        advantage: z.boolean().optional(),
        disadvantage: z.boolean().optional(),
      },
      annotations: {
        idempotentHint: true,
        readOnlyHint: true,
        destructiveHint: false,
        openWorldHint: false,
      },
    },
    async ({ name, stat, advantage, disadvantage }) => {
      if (advantage) {
        return invokeDmToolApi(
          `/v1/monsters/${name}/roll/stat/${stat}?advantage`,
        );
      }
      if (disadvantage) {
        return invokeDmToolApi(
          `/v1/monsters/${name}/roll/stat/${stat}?disadvantage`,
        );
      }
      return invokeDmToolApi(`/v1/monsters/${name}/roll/stat/${stat}`);
    },
  );

  server.registerTool(
    'roll_monster_attack',
    {
      title: 'Roll Monster Attack',
      description: `Rolls a monster's attack roll. Optionally can be rolled with advantage or disadvantage`,
      inputSchema: {
        name: z.string(),
        index: z.number(),
        advantage: z.boolean().optional(),
        disadvantage: z.boolean().optional(),
      },
      annotations: {
        idempotentHint: true,
        readOnlyHint: true,
        destructiveHint: false,
        openWorldHint: false,
      },
    },
    async ({ name, index, advantage, disadvantage }) => {
      if (advantage) {
        return invokeDmToolApi(
          `/v1/monsters/${name}/roll/attack/${index}?advantage`,
        );
      }
      if (disadvantage) {
        return invokeDmToolApi(
          `/v1/monsters/${name}/roll/attack/${index}?disadvantage`,
        );
      }
      return invokeDmToolApi(`/v1/monsters/${name}/roll/attack/${index}`);
    },
  );

  server.registerTool(
    'roll_monster_damage',
    {
      title: 'Roll Monster Damage',
      description: `Rolls a monster's damage roll. Optionally can be rolled as a critical hit`,
      inputSchema: {
        name: z.string(),
        index: z.number(),
        critical: z.boolean().optional(),
      },
      annotations: {
        idempotentHint: true,
        readOnlyHint: true,
        destructiveHint: false,
        openWorldHint: false,
      },
    },
    async ({ name, index, critical }) => {
      if (critical) {
        return invokeDmToolApi(
          `/v1/monsters/${name}/roll/damage/${index}?critical`,
        );
      }
      return invokeDmToolApi(`/v1/monsters/${name}/roll/damage/${index}`);
    },
  );

  const transport = new StdioServerTransport();
  await server.connect(transport);
})();
