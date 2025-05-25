import { AttackRoll, RawMonster, TransformedMonster } from '../constants/types';
import { transformOllamaToArray } from '../utilities/transform-ollama-to-array';
import { isAttackRoll } from '../constants/types.guard';
import { splitHtml } from '../utilities/split-html';

export const transformActions = async (
  monster: RawMonster,
): Promise<TransformedMonster['actions']> => {
  const rawActions = monster.Actions;
  if (!rawActions) {
    return;
  }
  return {
    list: splitHtml(rawActions),
    attackRolls: await getAttackRolls(rawActions),
  };
};

const getAttackRolls = (rawActions: string): Promise<AttackRoll[]> =>
  transformOllamaToArray({
    rawText: rawActions,
    typeGuard: isAttackRoll,
    examples: [
      {
        input:
          '<p><em><strong>Multiattack.</strong></em> The merrow makes two attacks: one with its bite and one with its claws or harpoon. </p><p><em><strong>Bite.</strong></em> <em>Melee Weapon Attack:</em> +6 to hit, reach 5 ft., one target. <em>Hit:</em> 8 (1d8 + 4) piercing damage. </p><p><em><strong>Claws.</strong></em> <em>Melee Weapon Attack:</em> +6 to hit, reach 5 ft., one target. <em>Hit:</em> 9 (2d4 + 4) slashing damage. </p><p><em><strong>Harpoon.</strong></em> <em>Melee or <em>Ranged Weapon Attack:</em></em> +6 to hit, reach 5 ft. or range 20/60 ft., one target. <em>Hit:</em> 11 (2d6 + 4) piercing damage. If the target is a Huge or smaller creature, it must succeed on a Strength contest against the merrow or be pulled up to 20 feet toward the merrow.</p>',
        parsed: [
          {
            name: 'bite',
            attackType: 'meleeWeapon',
            reach: 5,
            hit: 6,
            damage: [
              {
                roll: '1d8+4',
                damageType: 'piercing',
              },
            ],
          },
          {
            name: 'claws',
            attackType: 'meleeWeapon',
            reach: 5,
            hit: 6,
            damage: [
              {
                roll: '2d4+4',
                damageType: 'slashing',
              },
            ],
          },
          {
            name: 'harpoon',
            attackType: 'meleeWeapon',
            reach: 5,
            hit: 6,
            damage: [
              {
                roll: '2d6+4',
                damageType: 'piercing',
              },
            ],
          },
          {
            name: 'harpoon',
            attackType: 'rangedWeapon',
            reach: 20,
            hit: 6,
            damage: [
              {
                roll: '2d6+4',
                damageType: 'piercing',
              },
            ],
          },
        ],
      },
      {
        input:
          "<p><em><strong>Multiattack.</strong></em> The dragon makes three attacks: one with its bite and two with its claws. </p><p><em><strong>Bite.</strong></em> <em>Melee Weapon Attack:</em> +7 to hit, reach 10 ft., one target. <em>Hit:</em> 15 (2d10 + 4) piercing damage. </p><p><em><strong>Claw.</strong></em> <em>Melee Weapon Attack:</em> +7 to hit, reach 5 ft., one target. <em>Hit:</em> 11 (2d6 + 4) slashing damage. </p><p><em><strong>Breath Weapons (Recharge 5–6).</strong></em> The dragon uses one of the following breath weapons. </p><p><em><strong>Acid Breath.</strong></em> The dragon exhales acid in an 40-foot line that is 5 feet wide. Each creature in that line must make a DC 14 Dexterity saving throw, taking 40 (9d8) acid damage on a failed save, or half as much damage on a successful one. </p><p><em><strong>Slowing Breath.</strong></em> The dragon exhales gas in a 30-foot cone. Each creature in that area must succeed on a DC 14 Constitution saving throw. On a failed save, the creature can't use reactions, its speed is halved, and it can't make more than one attack on its turn. In addition, the creature can use either an action or a bonus action on its turn, but not both. These effects last for 1 minute. The creature can repeat the saving throw at the end of each of its turns, ending the effect on itself with a successful save.</p>",
        parsed: [
          {
            name: 'bite',
            attackType: 'meleeWeapon',
            reach: 10,
            hit: 7,
            damage: [
              {
                roll: '2d10+4',
                damageType: 'piercing',
              },
            ],
          },
          {
            name: 'claw',
            attackType: 'meleeWeapon',
            reach: 5,
            hit: 7,
            damage: [
              {
                roll: '2d6+4',
                damageType: 'slashing',
              },
            ],
          },
        ],
      },
      {
        input:
          "<p><em><strong>Multiattack. (Vampire Form Only).</strong></em> The vampire makes two attacks, only one of which can be a bite attack. </p><p><em><strong>Unarmed Strike (Vampire Form Only).</strong></em> <em>Melee Weapon Attack:</em> +9 to hit, reach 5 ft., one creature. <em>Hit:</em> 8 (1d8 + 4) bludgeoning damage. Instead of dealing damage, the vampire can grapple the target (escape DC 18). </p><p><em><strong>Bite. (Bat or Vampire Form Only).</strong></em> <em>Melee Weapon Attack:</em> +9 to hit, reach 5 ft., one willing creature, or a creature that is grappled by the vampire, incapacitated, or restrained. <em>Hit:</em> 7 (1d6 + 4) piercing damage plus 10 (3d6) necrotic damage. The target's hit point maximum is reduced by an amount equal to the necrotic damage taken, and the vampire regains hit points equal to that amount. The reduction lasts until the target finishes a long rest. The target dies if this effect reduces its hit point maximum to 0. A humanoid slain in this way and then buried in the ground rises the following night as a vampire spawn under the vampire's control. </p><p><em><strong>Charm.</strong></em> The vampire targets one humanoid it can see within 30 feet of it. If the target can see the vampire, the target must succeed on a DC 17 Wisdom saving throw against this magic or be charmed by the vampire. The charmed target regards the vampire as a trusted friend to be heeded and protected. Although the target isn't under the vampire's control, it takes the vampire's requests or actions in the most favorable way it can, and it is a willing target for the vampire's bite attack.</p><p>Each time the vampire or the vampire's companions do anything harmful to the target, it can repeat the saving throw, ending the effect on itself on a success. Otherwise, the effect lasts 24 hours or until the vampire is destroyed, is on a different plane of existence than the target, or takes a bonus action to end the effect. </p><p><em><strong>Children of the Night (1/Day).</strong></em> The vampire magically calls 2d4 swarms of bats or rats (swarm of bats, swarm of rats), provided that the sun isn't up. While outdoors, the vampire can call 3d6 wolves (wolf) instead. The called creatures arrive in 1d4 rounds, acting as allies of the vampire and obeying its spoken commands. The beasts remain for 1 hour, until the vampire dies, or until the vampire dismisses them as a bonus action.</p>",
        parsed: [
          {
            name: 'unarmedStrike',
            attackType: 'meleeWeapon',
            reach: 5,
            hit: 9,
            damage: [
              {
                roll: '1d8+4',
                damageType: 'bludgeoning',
              },
            ],
          },
          {
            name: 'bite',
            attackType: 'meleeWeapon',
            reach: 5,
            hit: 9,
            damage: [
              {
                roll: '1d6+4',
                damageType: 'piercing',
              },
              {
                roll: '3d6',
                damageType: 'necrotic',
              },
            ],
          },
        ],
      },
      {
        input:
          '<p><em><strong>Multiattack.</strong></em> The veteran makes two longsword attacks. If it has a shortsword drawn, it can also make a shortsword attack. </p><p><em><strong>Longsword.</strong></em> <em>Melee Weapon Attack:</em> +5 to hit, reach 5 ft., one target. <em>Hit:</em> 7 (1d8 + 3) slashing damage, or 8 (1d10 + 3) slashing damage if used with two hands. </p><p><em><strong>Shortsword.</strong></em> <em>Melee Weapon Attack:</em> +5 to hit, reach 5 ft., one target. <em>Hit:</em> 6 (1d6 + 3) piercing damage. </p><p><em><strong>Heavy Crossbow.</strong></em> <em>Ranged Weapon Attack:</em> +3 to hit, range 100/400 ft., one target. <em>Hit:</em> 6 (1d10 + 1) piercing damage.</p>',
        parsed: [
          {
            name: 'longsword',
            attackType: 'meleeWeapon',
            reach: 5,
            hit: 5,
            damage: [
              {
                damageType: 'slashing',
                roll: '1d8+3',
              },
            ],
          },
          {
            name: 'shortsword',
            attackType: 'meleeWeapon',
            reach: 5,
            hit: 5,
            damage: [
              {
                damageType: 'piercing',
                roll: '1d6+3',
              },
            ],
          },
          {
            name: 'heavyCrossbow',
            attackType: 'rangedWeapon',
            reach: 100,
            hit: 3,
            damage: [
              {
                damageType: 'piercing',
                roll: '1d10+1',
              },
            ],
          },
        ],
      },
    ],
  }).then((result) => {
    if (result.type === 'some') {
      return result.some;
    }
    return [];
  });
