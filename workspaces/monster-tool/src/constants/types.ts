/** @see {isRawMonster} ts-auto-guard:type-guard */
export interface RawMonster {
  readonly name: string;
  readonly meta: string;
  readonly 'Armor Class': string;
  readonly 'Hit Points': string;
  readonly Speed: string;
  readonly STR: string;
  readonly STR_mod: string;
  readonly DEX: string;
  readonly DEX_mod: string;
  readonly CON: string;
  readonly CON_mod: string;
  readonly INT: string;
  readonly INT_mod: string;
  readonly WIS: string;
  readonly WIS_mod: string;
  readonly CHA: string;
  readonly CHA_mod: string;
  readonly 'Saving Throws'?: string;
  readonly Skills?: string;
  readonly Senses: string;
  readonly Languages: string;
  readonly Challenge: string;
  readonly Traits?: string;
  readonly Actions?: string;
  readonly Reactions?: string;
  readonly 'Damage Vulnerabilities'?: string;
  readonly 'Legendary Actions'?: string;
  readonly 'Damage Immunities'?: string;
  readonly 'Damage Resistances'?: string;
  readonly 'Condition Immunities'?: string;
  readonly img_url: string;
}

export interface TransformedMonster {
  readonly name: string;
  readonly ac: number;
  readonly size: string;
  readonly creatureType: string;
  readonly alignment: string;
  readonly languages: string[];
  readonly maxHitPoints: number;
  readonly hitDice: string;
  readonly speed: {
    readonly walk: number;
    readonly fly: number;
    readonly swim: number;
    readonly burrow: number;
    readonly climb: number;
    readonly hover: boolean;
  };
  readonly modifiers: {
    readonly str: number;
    readonly dex: number;
    readonly con: number;
    readonly int: number;
    readonly wis: number;
    readonly cha: number;
  };
  readonly stats: {
    readonly str: number;
    readonly dex: number;
    readonly con: number;
    readonly int: number;
    readonly wis: number;
    readonly cha: number;
  };
  readonly imageUrl: string;
}
