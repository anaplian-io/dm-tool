# dm-tool

A lightweight suite of utilities designed to help Dungeon Masters run, track, and enrich Dungeons & Dragons encounters.

## Installation

Installation steps:

1. Clone the repository.
2. Ensure you have Node≥22 and the Rust toolchain (cargo) installed.
3. Run `npm install` to install npm dependencies.
4. Build the project with `npm run build`.

## Routes

### API Routes

The server runs on `http://localhost:8080/v1`. Main endpoints:

| Method | Path                                           | Description                                                      |
| ------ | ---------------------------------------------- | ---------------------------------------------------------------- |
| GET    | `/dice/roll/{expression}`                      | Roll dice using the expression syntax (e.g., `2d6+3`).           |
| GET    | `/dice/list`                                   | List supported dice.                                             |
| GET    | `/monsters`                                    | Retrieve the full list of monsters loaded from `monsters.json`.  |
| GET    | `/monsters/{monster_name}`                     | Get detailed information for a specific monster.                 |
| GET    | `/monsters/{monster_name}/roll/stat/{stat}`    | Roll a saving‑throw, skill or ability score for the monster.     |
| GET    | `/monsters/{monster_name}/roll/skill/{skill}`  | Same as above but for a skill name.                              |
| GET    | `/monsters/{monster_name}/roll/attack/{index}` | Perform an attack roll using the monster's indexed attack entry. |
| GET    | `/monsters/{monster_name}/roll/damage/{index}` | Roll damage for the monster's indexed attack.                    |

## Model Context Protocol

### Model Context Protocol (MCP)

On build, an `mcp.json` file is generated in to help you install dm‑tool as an MCP server.

```json
{
  "dm-tool": {
    "command": "node",
    "args": ["/path/to/workspaces/mcp-server/dist/index.js"]
  }
}
```

This enables AI assistants to run the tool programmatically.

## Development

- Run tests with `npm run test`.
- Fix all formatting issues with `npm run format`.
