import { RawMonster, TransformedMonster } from '../constants/types';

export const transformStats = (
  monster: RawMonster,
): TransformedMonster['stats'] => ({
  str: parseInt(monster.STR, 10),
  dex: parseInt(monster.DEX, 10),
  con: parseInt(monster.CON, 10),
  int: parseInt(monster.INT, 10),
  wis: parseInt(monster.WIS, 10),
  cha: parseInt(monster.CHA, 10),
});
