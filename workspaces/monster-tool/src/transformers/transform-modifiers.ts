import { RawMonster, TransformedMonster } from '../constants/types';

export const transformModifiers = (
  monster: RawMonster,
): TransformedMonster['modifiers'] => ({
  str: extract(monster.STR_mod),
  dex: extract(monster.DEX_mod),
  con: extract(monster.CON_mod),
  int: extract(monster.INT_mod),
  wis: extract(monster.WIS_mod),
  cha: extract(monster.CHA_mod),
});

const regex = /[+-]?\d+/;
const extract = (rawModifier: string): number => {
  const match = rawModifier.trim().match(regex);
  return match
    ? parseInt(match[0], 10) * (rawModifier.startsWith('-') ? -1 : 1)
    : 0;
};
