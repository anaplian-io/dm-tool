import { RawMonster } from '../constants/types';

export const transformHitDice = (monster: RawMonster): string => {
  const match = monster['Hit Points'].match(/\s*(\d+d\d+)\s*/);
  return match?.[1] ?? '';
};
