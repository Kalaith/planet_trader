import type { Planet } from '../types/entities';

const PLANET_TYPE_CLASSES: Record<string, string> = {
  rocky: 'bg-stone-500',
  'ice world': 'bg-sky-200',
  volcanic: 'bg-red-600',
  desert: 'bg-yellow-600',
  'gas dwarf': 'bg-violet-500',
  tundra: 'bg-teal-200',
  'ocean world': 'bg-cyan-500',
  'lava crust': 'bg-orange-600',
  'clouded giant': 'bg-indigo-400',
  crystalline: 'bg-blue-200',
  'swamp world': 'bg-lime-700',
  'tidally locked': 'bg-amber-500',
  'hollow planet': 'bg-zinc-500',
  'bio-waste zone': 'bg-green-700',
  'irradiated core': 'bg-yellow-400',
};

export const getPlanetSurfaceClass = (planet: Planet): string => {
  const typeName = planet.type?.name?.toLowerCase();
  return typeName ? (PLANET_TYPE_CLASSES[typeName] ?? 'bg-slate-500') : 'bg-slate-500';
};

export const getPlanetAtmosphereClass = (planet: Planet): string => {
  if (planet.atmosphere >= 2) {
    return 'opacity-90';
  }
  if (planet.atmosphere >= 1) {
    return 'opacity-60';
  }
  if (planet.atmosphere >= 0.35) {
    return 'opacity-30';
  }

  return 'opacity-0';
};

export const getPlanetWaterClass = (planet: Planet): string => {
  if (planet.water >= 0.75) {
    return 'opacity-90';
  }
  if (planet.water >= 0.45) {
    return 'opacity-60';
  }
  if (planet.water >= 0.15) {
    return 'opacity-30';
  }

  return 'opacity-0';
};
