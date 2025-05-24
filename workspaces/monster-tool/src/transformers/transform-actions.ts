import {
  MultiAttack,
  RawMonster,
  TransformedMonster,
} from '../constants/types';
import { transformOllamaToArray } from '../utilities/transform-ollama-to-array';
import { isMultiAttack } from '../constants/types.guard';

export const transformActions = async (
  monster: RawMonster,
): Promise<TransformedMonster['actions']> => {
  const rawActions = monster.Actions;
  if (!rawActions) {
    return;
  }
  return {
    raw: rawActions,
    sanitized: rawActions.replace(/<[^>]*>/g, ''),
    multiAttack: await getMultiAttack(rawActions),
  };
};

const getMultiAttack = (rawActions: string): Promise<MultiAttack[]> => {
  if (!rawActions.toLowerCase().match('multiattack')) {
    return Promise.resolve([]);
  }
  return transformOllamaToArray({
    rawText: rawActions,
    typeGuard: isMultiAttack,
    examples: [
      {
        input:
          "<p><em><strong>Multiattack.</strong></em> The aboleth makes three tentacle attacks. </p><p><em><strong>Tentacle.</strong></em> <em>Melee Weapon Attack:</em> +9 to hit, reach 10 ft., one target. <em>Hit:</em> 12 (2d6 + 5) bludgeoning damage. If the target is a creature, it must succeed on a DC 14 Constitution saving throw or become diseased. The disease has no effect for 1 minute and can be removed by any magic that cures disease. After 1 minute, the diseased creature's skin becomes translucent and slimy, the creature can't regain hit points unless it is underwater, and the disease can be removed only by heal or another disease-curing spell of 6th level or higher. When the creature is outside a body of water, it takes 6 (1d12) acid damage every 10 minutes unless moisture is applied to the skin before 10 minutes have passed. </p><p><em><strong>Tail.</strong></em> <em>Melee Weapon Attack:</em> +9 to hit, reach 10 ft. one target. <em>Hit:</em> 15 (3d6 + 5) bludgeoning damage. </p><p><em><strong>Enslave (3/Day).</strong></em> The aboleth targets one creature it can see within 30 feet of it. The target must succeed on a DC 14 Wisdom saving throw or be magically charmed by the aboleth until the aboleth dies or until it is on a different plane of existence from the target. The charmed target is under the aboleth's control and can't take reactions, and the aboleth and the target can communicate telepathically with each other over any distance. </p><p>Whenever the charmed target takes damage, the target can repeat the saving throw. On a success, the effect ends. No more than once every 24 hours, the target can also repeat the saving throw when it is at least 1 mile away from the aboleth.</p>",
        parsed: [
          {
            name: 'tentacle',
            count: 3,
          },
        ],
      },
      {
        input:
          '<p><em><strong>Club.</strong></em> <em>Melee Weapon Attack:</em> +2 to hit, reach 5 ft., one target. <em>Hit:</em> 2 (1d4) bludgeoning damage.</p>',
        parsed: [],
      },
      {
        input:
          "<p><em><strong>Multiattack.</strong></em> The dragon can use its Frightful Presence. It then makes three attacks: one with its bite and two with its claws.</p><p><em><strong>Bite.</strong></em> <em>Melee Weapon Attack:</em> +11 to hit, reach 10 ft., one target. <em>Hit:</em> 17 (2d10 + 6) piercing damage plus 4 (1d8) acid damage.</p><p><em><strong>Claw.</strong></em> <em>Melee Weapon Attack:</em> +11 to hit, reach 5 ft., one target. <em>Hit:</em> 13 (2d6 + 6) slashing damage.</p><p><em><strong>Tail.</strong></em> <em>Melee Weapon Attack:</em> +11 to hit, reach 15 ft., one target. <em>Hit:</em> 15 (2d8 + 6) bludgeoning damage.</p><p><em><strong>Frightful Presence.</strong></em> Each creature of the dragon's choice that is within 120 feet of the dragon and aware of it must succeed on a DC 16 Wisdom saving throw or become frightened for 1 minute. A creature can repeat the saving throw at the end of each of its turns, ending the effect on itself on a success. If a creature's saving throw is successful or the effect ends for it, the creature is immune to the dragon's Frightful Presence for the next 24 hours.</p><p><em><strong>Acid Breath (Recharge 5–6).</strong></em> The dragon exhales acid in a 60-­foot line that is 5 feet wide. Each creature in that line must make a DC 18 Dexterity saving throw, taking 54 (12d8) acid damage on a failed save, or half as much damage on a successful one.</p>",
        parsed: [
          {
            name: 'bite',
            count: 1,
          },
          {
            name: 'claw',
            count: 2,
          },
        ],
      },
      {
        input:
          "<p><em><strong>Multiattack. (Vampire Form Only).</strong></em> The vampire makes two attacks, only one of which can be a bite attack. </p><p><em><strong>Unarmed Strike (Vampire Form Only).</strong></em> <em>Melee Weapon Attack:</em> +9 to hit, reach 5 ft., one creature. <em>Hit:</em> 8 (1d8 + 4) bludgeoning damage. Instead of dealing damage, the vampire can grapple the target (escape DC 18). </p><p><em><strong>Bite. (Bat or Vampire Form Only).</strong></em> <em>Melee Weapon Attack:</em> +9 to hit, reach 5 ft., one willing creature, or a creature that is grappled by the vampire, incapacitated, or restrained. <em>Hit:</em> 7 (1d6 + 4) piercing damage plus 10 (3d6) necrotic damage. The target's hit point maximum is reduced by an amount equal to the necrotic damage taken, and the vampire regains hit points equal to that amount. The reduction lasts until the target finishes a long rest. The target dies if this effect reduces its hit point maximum to 0. A humanoid slain in this way and then buried in the ground rises the following night as a vampire spawn under the vampire's control. </p><p><em><strong>Charm.</strong></em> The vampire targets one humanoid it can see within 30 feet of it. If the target can see the vampire, the target must succeed on a DC 17 Wisdom saving throw against this magic or be charmed by the vampire. The charmed target regards the vampire as a trusted friend to be heeded and protected. Although the target isn't under the vampire's control, it takes the vampire's requests or actions in the most favorable way it can, and it is a willing target for the vampire's bite attack.</p><p>Each time the vampire or the vampire's companions do anything harmful to the target, it can repeat the saving throw, ending the effect on itself on a success. Otherwise, the effect lasts 24 hours or until the vampire is destroyed, is on a different plane of existence than the target, or takes a bonus action to end the effect. </p><p><em><strong>Children of the Night (1/Day).</strong></em> The vampire magically calls 2d4 swarms of bats or rats (swarm of bats, swarm of rats), provided that the sun isn't up. While outdoors, the vampire can call 3d6 wolves (wolf) instead. The called creatures arrive in 1d4 rounds, acting as allies of the vampire and obeying its spoken commands. The beasts remain for 1 hour, until the vampire dies, or until the vampire dismisses them as a bonus action.</p>",
        parsed: [
          {
            name: 'bite',
            count: 1,
          },
          {
            name: 'unarmedStrike',
            count: 1,
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
};
