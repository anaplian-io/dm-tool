export const extractFirstNumber = (expression: string): number => {
  const match = expression.match(/(\d+)/);
  return match ? parseInt(match[1] ?? '0', 10) : 0;
};
