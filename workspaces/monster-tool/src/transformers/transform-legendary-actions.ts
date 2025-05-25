import { RawMonster, TransformedMonster } from '../constants/types';
import { splitHtml } from '../utilities/split-html';

export const transformLegendaryActions = (
  monster: RawMonster,
): TransformedMonster['legendaryActions'] =>
  monster['Legendary Actions'] ? splitHtml(monster['Legendary Actions']) : [];
