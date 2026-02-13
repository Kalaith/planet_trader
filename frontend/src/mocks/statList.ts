// Mock data for statList
import type { Planet } from "../types/entities";

const statList: Array<{
  label: string;
  id: keyof Planet;
  unit: string;
  bar: (p: Planet) => number;
}> = [
  {
    label: "Temperature",
    id: "temperature",
    unit: "°C",
    bar: (p) => Math.max(0, Math.min(100, ((p.temperature + 50) / 150) * 100)),
  },
  {
    label: "Atmosphere",
    id: "atmosphere",
    unit: "x",
    bar: (p) => Math.min(100, (p.atmosphere / 2) * 100),
  },
  { label: "Water", id: "water", unit: "%", bar: (p) => (p.water || 0) * 100 },
  {
    label: "Gravity",
    id: "gravity",
    unit: "x",
    bar: (p) => Math.min(100, (p.gravity / 3) * 100),
  },
  {
    label: "Radiation",
    id: "radiation",
    unit: "x",
    bar: (p) => Math.min(100, (p.radiation / 1.5) * 100),
  },
];

export default statList;
