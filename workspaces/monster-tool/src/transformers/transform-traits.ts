import { RawMonster, TransformedMonster } from '../constants/types';

export const transformTraits = (
  monster: RawMonster,
): TransformedMonster['traits'] =>
  monster.Traits?.split('</p>')
    .map((rawTrait) => rawTrait.replaceAll(/<[^>]*>/g, '').trim())
    .filter((trait) => '' !== trait) ?? [];
