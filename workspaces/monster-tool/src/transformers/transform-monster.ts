import { RawMonster, TransformedMonster } from '../constants/types';
import { transformModifiers } from './transform-modifiers';
import { transformSpeed } from './transform-speed';
import { transformAc } from './transform-ac';
import { transformMaxHitPoints } from './transform-max-hit-points';
import { transformHitDice } from './transform-hit-dice';
import { transformSize } from './transform-size';
import { transformCreatureType } from './transform-creature-type';
import { transformAlignment } from './transform-alignment';
import { transformStats } from './transform-stats';
import { transformLanguages } from './transform-languages';
import { transformSavingThrows } from './transform-saving-throws';
import { transformSkills } from './transform-skills';
import { transformTraits } from './transform-traits';

export const transformMonster = (monster: RawMonster): TransformedMonster => ({
  name: monster.name,
  ac: transformAc(monster),
  size: transformSize(monster),
  creatureType: transformCreatureType(monster),
  alignment: transformAlignment(monster),
  languages: transformLanguages(monster),
  maxHitPoints: transformMaxHitPoints(monster),
  hitDice: transformHitDice(monster),
  speed: transformSpeed(monster),
  modifiers: transformModifiers(monster),
  stats: transformStats(monster),
  savingThrows: transformSavingThrows(monster),
  skills: transformSkills(monster),
  traits: transformTraits(monster),
  imageUrl: monster.img_url,
});
