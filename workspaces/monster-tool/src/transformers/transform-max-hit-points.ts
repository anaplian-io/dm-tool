import { RawMonster } from '../constants/types';
import { extractFirstNumber } from '../utilities/extract-first-number';

export const transformMaxHitPoints = (monster: RawMonster): number =>
  extractFirstNumber(monster['Hit Points']);
