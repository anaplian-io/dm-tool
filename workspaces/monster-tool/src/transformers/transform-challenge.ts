import { RawMonster, TransformedMonster } from '../constants/types';

export const transformChallenge = (
  monster: RawMonster,
): TransformedMonster['challenge'] => {
  const challengeRatingExpression = monster.Challenge.trim().split(' (');
  return {
    rating: challengeRatingExpression[0] ?? '0',
    xp: parseInt(
      challengeRatingExpression[1]
        ?.replaceAll(',', '')
        .replaceAll('XP)', '')
        .trim() ?? '0',
    ),
  };
};
