import { FIVE_E_MONSTERS_URL } from './constants/general';
import { isRawMonster } from './constants/types.guard';
import { isArray } from './utilities/guards';
import { TransformedMonster } from './constants/types';
import { transformMonster } from './transformers/transform-monster';
import * as fs from 'node:fs';

(async () => {
  console.info(`Fetching monsters from ${FIVE_E_MONSTERS_URL}`);
  const rawResponseText = await fetch(FIVE_E_MONSTERS_URL).then((result) =>
    result.text(),
  );
  const rawParsedMonsters: unknown = JSON.parse(rawResponseText);
  if (!isArray(rawParsedMonsters)) {
    console.error('Did not receive an array; could not parse');
    process.exit(1);
  }
  const validMonsters = rawParsedMonsters.filter(isRawMonster);
  console.info(
    `Discovered ${validMonsters.length} valid monsters out of ${rawParsedMonsters.length} total`,
  );
  const transformedMonsters: TransformedMonster[] =
    validMonsters.map(transformMonster);
  fs.writeFileSync(
    'monsters.json',
    JSON.stringify(transformedMonsters, null, 2),
  );
})();
