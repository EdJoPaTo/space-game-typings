/**
 * Space game typings
 * by EdJoPaTo
 * AGPL-3.0-or-later
 * https://github.com/EdJoPaTo/space-game-typings
 */

export function isSiteEntityAsteroid(
  entity: SiteEntity,
): entity is SiteEntityAsteroid {
  return "ore" in entity;
}
export function isSiteEntityFacility(
  entity: SiteEntity,
): entity is SiteEntityFacility {
  return "facility" in entity;
}
export function isSiteEntityNpc(entity: SiteEntity): entity is SiteEntityNpc {
  return "faction" in entity;
}
export function isSiteEntityPlayer(
  entity: SiteEntity,
): entity is SiteEntityPlayer {
  return "player" in entity;
}

export function isPlayerLocationSite(
  location: PlayerLocation,
): location is PlayerLocationSite {
  return "site" in location;
}
export function isPlayerLocationStation(
  location: PlayerLocation,
): location is PlayerLocationStation {
  return "station" in location;
}
export function isPlayerLocationWarp(
  location: PlayerLocation,
): location is PlayerLocationWarp {
  return "towards" in location;
}

/* Autogenerated after here */
