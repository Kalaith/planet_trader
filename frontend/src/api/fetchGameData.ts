import { randomItem } from '../game/useGame';
import type { Species } from '../types/entities';
import { apiClient } from './apiClient';
import { ApiResponse, ApiError } from './types';

const baseUrl = 'mocks';

interface AlienSpeciesTypeTemplate {
  prefixes: string[];
  suffixes: string[];
  desc: string;
  temp: [number, number];
  atmo: [number, number];
  water: [number, number];
  grav: [number, number];
  rad: [number, number];
  colors: string[];
}

export const fetchGameData = async () => {
  try {
    const [
      planetTypesRes,
      alienSpeciesRes,
      terraformingToolsRes,
      planetNamesRes,
      alienSpeciesTypesRes
    ] = await Promise.all([
      apiClient.get(`${baseUrl}/planet_types.json`),
      apiClient.get(`${baseUrl}/alien_species.json`),
      apiClient.get(`${baseUrl}/terraforming_tools.json`),
      apiClient.get(`${baseUrl}/planet_names.json`),
      apiClient.get(`${baseUrl}/alien_species_types.json`),
    ]);

    const planetTypes = planetTypesRes.data;
    const alienSpecies = alienSpeciesRes.data;
    const terraformingTools = terraformingToolsRes.data;
    const planetNames = planetNamesRes.data;
    const alienSpeciesTypes = alienSpeciesTypesRes.data;

    const mappedAlienSpeciesTypes: Species[] = (
      alienSpeciesTypes as AlienSpeciesTypeTemplate[]
    ).map(species => ({
      name: `${randomItem(species.prefixes)} ${randomItem(species.suffixes)}`,
      description: species.desc,
      tempRange: species.temp,
      atmoRange: species.atmo,
      waterRange: species.water,
      gravRange: species.grav,
      radRange: species.rad,
      basePrice: Math.floor(Math.random() * 1000 + 500),
      color: randomItem(species.colors),
    }));

    return new ApiResponse({
      success: true,
      data: {
        planetTypes,
        alienSpecies,
        terraformingTools,
        planetNames,
        alienSpeciesTypes: mappedAlienSpeciesTypes,
      }
    });
  } catch (error) {
    console.error('Failed to fetch game data:', error);
    throw new ApiError('Failed to fetch game data', 500);
  }
};
