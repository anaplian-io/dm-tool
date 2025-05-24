import { RawMonster, TransformedMonster } from '../constants/types';
import { getMapFromStatList } from '../utilities/get-map-from-stat-list';

export const transformSavingThrows = (
  monster: RawMonster,
): TransformedMonster['savingThrows'] => {
  const savingThrowsExpression = monster['Saving Throws'];
  if (!savingThrowsExpression) {
    return {
      str: 0,
      dex: 0,
      con: 0,
      int: 0,
      wis: 0,
      cha: 0,
    };
  }
  const throwMap = getMapFromStatList(savingThrowsExpression);
  return {
    str: throwMap['str'] ?? 0,
    dex: throwMap['dex'] ?? 0,
    con: throwMap['con'] ?? 0,
    int: throwMap['int'] ?? 0,
    wis: throwMap['wis'] ?? 0,
    cha: throwMap['cha'] ?? 0,
  };
};
