import { RawMonster, TransformedMonster } from '../constants/types';
import { getMapFromStatList } from '../utilities/get-map-from-stat-list';

export const transformSkills = (
  monster: RawMonster,
): TransformedMonster['skills'] => {
  const skillsExpression = monster.Skills;
  if (!skillsExpression) {
    return {
      acrobatics: 0,
      animalHandling: 0,
      arcana: 0,
      athletics: 0,
      deception: 0,
      history: 0,
      insight: 0,
      intimidation: 0,
      investigation: 0,
      medicine: 0,
      nature: 0,
      perception: 0,
      performance: 0,
      persuasion: 0,
      religion: 0,
      sleightOfHand: 0,
      stealth: 0,
      survival: 0,
    };
  }
  const skillMap = getMapFromStatList(skillsExpression);
  return {
    acrobatics: skillMap['acrobatics'] ?? 0,
    animalHandling: skillMap['animalHandling'] ?? 0,
    arcana: skillMap['arcana'] ?? 0,
    athletics: skillMap['athletics'] ?? 0,
    deception: skillMap['deception'] ?? 0,
    history: skillMap['history'] ?? 0,
    insight: skillMap['insight'] ?? 0,
    intimidation: skillMap['intimidation'] ?? 0,
    investigation: skillMap['investigation'] ?? 0,
    medicine: skillMap['medicine'] ?? 0,
    nature: skillMap['nature'] ?? 0,
    perception: skillMap['perception'] ?? 0,
    performance: skillMap['performance'] ?? 0,
    persuasion: skillMap['persuasion'] ?? 0,
    religion: skillMap['religion'] ?? 0,
    sleightOfHand: skillMap['sleightOfHand'] ?? 0,
    stealth: skillMap['stealth'] ?? 0,
    survival: skillMap['survival'] ?? 0,
  };
};
