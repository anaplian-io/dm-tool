import { RawMonster, TransformedMonster } from '../constants/types';

export const transformReactions = (
  monster: RawMonster,
): TransformedMonster['reactions'] =>
  monster.Reactions ? { raw: monster.Reactions } : undefined;
