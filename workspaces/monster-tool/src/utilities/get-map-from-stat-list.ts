export const getMapFromStatList = (statList: string): Record<string, number> =>
  Object.fromEntries<number>(
    statList
      .split(',')
      .map((entry) => entry.trim().split(' '))
      .map((entry) => [
        entry[0]?.toLowerCase() ?? '_',
        parseInt(entry[1] ?? '0', 10),
      ]),
  );
