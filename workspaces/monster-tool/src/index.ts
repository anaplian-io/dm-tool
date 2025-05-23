import { FIVE_E_MONSTERS_URL } from './constants/general';
import { isRawMonster } from './constants/types.guard';

(async () => {
  console.info(`Fetching monsters from ${FIVE_E_MONSTERS_URL}`);
  const rawResponseText = await fetch(FIVE_E_MONSTERS_URL).then((result) =>
    result.text(),
  );
  const rawParsedMonsters = JSON.parse(rawResponseText) as unknown[];
  if (!Array.isArray(rawParsedMonsters)) {
    console.error('Did not receive an array; could not parse');
    process.exit(1);
  }
  const validMonsters = rawParsedMonsters.filter(isRawMonster);
  console.info(
    `Discovered ${validMonsters.length} valid monsters out of ${rawParsedMonsters.length} total`,
  );
})();
