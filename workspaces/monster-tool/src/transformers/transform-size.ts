import { RawMonster } from '../constants/types';

export const transformSize = (monster: RawMonster): string => {
  return monster.meta.split(' ')[0]?.toLowerCase() ?? '';
};
