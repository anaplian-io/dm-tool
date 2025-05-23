import { RawMonster } from '../constants/types';

export const transformLanguages = (monster: RawMonster): string[] =>
  monster.Languages.split(',').map((language) => language.toLowerCase().trim());
