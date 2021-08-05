/**
 * Space game typings
 * by EdJoPaTo
 * AGPL-3.0-or-later
 * TODO: repolink
 */

export type LifelessIdentifier = `lifeless${string}`;
export type ModulePassiveIdentifier = `modp${string}`;
export type ModuleUntargetedIdentifier = `modu${string}`;
export type ModuleTargetedIdentifier = `modt${string}`;
export type SolarsystemIdentifier = string;

/** Example: player-tg-1337 */
export type PlayerIdentifier = `player-${string}-${number}`;
export type PlayerTelegramIdentifier = `player-tg-${number}`;

export type SitesNearPlanet = Readonly<Record<number, readonly SiteInfo[]>>;

export type SiteEntity =
  | ({ type: "lifeless" } & SiteEntityLifeless)
  | ({ type: "facility" } & SiteEntityFacility)
  | ({ type: "npc" } & SiteEntityNpc)
  | ({ type: "player" } & SiteEntityPlayer);

export type Instruction =
  | ({ step: "untargeted"; type: "module" } & InstructionUntargetedModule)
  | ({ step: "targeted"; type: "module" } & InstructionTargetedModule)
  | ({ step: "targeted"; type: "facility" } & InstructionTargetedFacility)
  | ({ step: "movement"; type: "warp" } & InstructionMovementWarp)
  | ({ step: "movement"; type: "undock" } & InstructionMovementUndock);

/* Autogenerated after here */
