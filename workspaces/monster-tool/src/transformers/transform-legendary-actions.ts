import { RawMonster, TransformedMonster } from '../constants/types';

export const transformLegendaryActions = (
  monster: RawMonster,
): TransformedMonster['legendaryActions'] =>
  monster['Legendary Actions']
    ? {
        raw: monster['Legendary Actions'],
      }
    : undefined;
