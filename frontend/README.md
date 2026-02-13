# Planetary Terraforming Company: React Frontend

This is the React frontend for **Planetary Terraforming Company**, a strategic terraforming simulation game. Players purchase, modify, and sell planets to diverse alien species, each with unique environmental requirements. The frontend is designed for modularity, type safety, and a clean, mobile-friendly user experience.

## Features

- **Modern React + TypeScript**: Strict typing, modular code, and reusable components.
- **Context & Hooks**: Centralized game state using React Context and custom hooks. No prop drilling.
- **Alien Market**: Sell planets to unique alien buyers, each with specific habitat needs. Visual compatibility indicators and dynamic pricing.
- **Terraforming Tools**: Use and unlock tools to modify planetary stats (temperature, atmosphere, water, gravity, radiation).
- **Planet Inventory**: Manage your owned planets, select for terraforming or sale.
- **Visual Feedback**: Immediate UI updates for terraforming actions, sales, and tool usage.
- **Mobile-Friendly UI**: Responsive layout, touch-friendly scrollbars, and accessible controls.
- **Toast Notifications**: Animated, auto-dismissing messages for all major actions.
- **Mock Data & API Layer**: All game data is loaded from JSON mocks via a centralized API utility.

## Folder Structure

```
frontend/
  src/
    api/                # Data fetching utilities
    components/         # React components (AlienMarketPanel, TerraformingToolsPanel, etc.)
    contexts/           # GameContext for global state
    types/              # TypeScript entity/type definitions
    utils/              # Utility functions (color, etc.)
    styles/             # Global and custom CSS
  public/
    mocks/              # Mock data (planets, tools, etc.)
  README.md             # This file
```

## Getting Started

1. **Install dependencies:**
   ```sh
   npm install
   ```
2. **Start the development server:**
   ```sh
   npm start
   ```
3. **Open in browser:**
   Visit [http://localhost:3000](http://localhost:3000)

## Key Technologies

- React 18+
- TypeScript
- Tailwind CSS (with custom scrollbar and animation tweaks)
- Context API & custom hooks

## Gameplay Overview

- Buy planets, analyze their stats, and use terraforming tools to match alien buyer requirements.
- Each alien buyer has unique environmental needs (temperature, atmosphere, water, gravity, radiation).
- Sell planets for profit; maximize compatibility for higher prices (perfect fit: 4,000–6,000₵).
- Unlock and research new tools as you progress.
- All actions provide instant feedback and animated notifications.

## Accessibility & UX

- All interactive elements are keyboard and screen reader accessible.
- Scrollable panels use custom, touch-friendly scrollbars.
- Responsive design for desktop and mobile.

## Development Notes

- All types are in `src/types/entities.ts`.
- Utilities in `src/utils/`.
- Mock data in `public/mocks/`.
- All API/data fetching is centralized in `src/api/fetchGameData.ts`.
- To add new tools, planets, or species, update the relevant mock JSON files.

## License

MIT
