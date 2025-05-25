export const splitHtml = (raw: string): string[] =>
  raw
    .split('</p>')
    .map((rawTrait) => rawTrait.replaceAll(/<[^>]*>/g, '').trim())
    .filter((trait) => '' !== trait);
