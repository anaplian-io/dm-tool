import { RawMonster } from '../constants/types';
import { extractFirstNumber } from '../utilities/extract-first-number';

export const transformAc = (monster: RawMonster): number =>
  extractFirstNumber(monster['Armor Class']);
