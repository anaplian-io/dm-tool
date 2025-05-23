import { RawMonster } from '../constants/types';

export const transformCreatureType = (monster: RawMonster): string => {
  return monster.meta.split(',')[0]?.split(' ')[1]?.toLowerCase() ?? '';
};
