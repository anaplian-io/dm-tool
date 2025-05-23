import { RawMonster } from '../constants/types';

export const transformAlignment = (monster: RawMonster): string => {
  return monster.meta.split(',')[1]?.trim().toLowerCase() ?? '';
};
