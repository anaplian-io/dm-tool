import { RawMonster, TransformedMonster } from '../constants/types';

export const transformSpeed = (
  monster: RawMonster,
): TransformedMonster['speed'] => ({
  walk: extractWalkSpeed(monster.Speed),
  fly: extractFlySpeed(monster.Speed),
  swim: extractSwimSpeed(monster.Speed),
  burrow: extractBurrowSpeed(monster.Speed),
  climb: extractClimbSpeed(monster.Speed),
  hover: extractHover(monster.Speed),
});

const extractWalkSpeed = (speedExpression: string): number => {
  const match = speedExpression.match(/(\d+)\s*ft\.?/);
  return match ? parseInt(match[1] ?? '0', 10) : 0;
};

const extractFlySpeed = (speedExpression: string): number => {
  const match = speedExpression.match(/(\d+)\s*ft\.?,\s*fly\s*(\d+)\s*ft\.?/);
  return match ? parseInt(match[2] ?? '0', 10) : 0;
};

const extractSwimSpeed = (speedExpression: string): number => {
  const match = speedExpression.match(/(\d+)\s*ft\.?,\s*swim\s*(\d+)\s*ft\.?/);
  return match ? parseInt(match[2] ?? '0', 10) : 0;
};

const extractBurrowSpeed = (speedExpression: string): number => {
  const match = speedExpression.match(
    /(\d+)\s*ft\.?,\s*burrow\s*(\d+)\s*ft\.?/,
  );
  return match ? parseInt(match[2] ?? '0', 10) : 0;
};

const extractClimbSpeed = (speedExpression: string): number => {
  const match = speedExpression.match(/(\d+)\s*ft\.?,\s*climb\s*(\d+)\s*ft\.?/);
  return match ? parseInt(match[2] ?? '0', 10) : 0;
};

const extractHover = (speedExpression: string): boolean => {
  return speedExpression.includes('(hover)');
};
