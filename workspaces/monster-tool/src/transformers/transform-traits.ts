import { RawMonster, TransformedMonster } from '../constants/types';
import { splitHtml } from '../utilities/split-html';

export const transformTraits = (
  monster: RawMonster,
): TransformedMonster['traits'] =>
  monster.Traits ? splitHtml(monster.Traits) : [];
