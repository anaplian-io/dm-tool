import { RawMonster, TransformedMonster } from '../constants/types';
import { splitHtml } from '../utilities/split-html';

export const transformReactions = (
  monster: RawMonster,
): TransformedMonster['reactions'] =>
  monster.Reactions ? splitHtml(monster.Reactions) : [];
