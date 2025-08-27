# Planetary Terraforming Company

A strategic terraforming simulation game where players purchase barren planets, modify their environmental conditions using advanced terraforming tools, and sell them to diverse alien species with specific habitat requirements.

## 🎮 Game Overview

In **Planetary Terraforming Company**, you take on the role of an interstellar environmental engineer. Your mission is to:

- **Purchase planets** from the galactic market
- **Analyze planetary conditions** (temperature, atmosphere, water, gravity, radiation)
- **Apply terraforming tools** to modify planetary environments
- **Sell customized worlds** to alien species with unique habitat preferences
- **Maximize profits** through strategic terraforming and optimal sales timing

The game features scientifically-grounded terraforming mechanics, diverse alien species with realistic environmental requirements, and progressive tool unlocking as you build your terraforming business.

## 🌟 Key Features

### Alien Species & Environmental Preferences
- **Pyrothane Lizards**: Cold-blooded species preferring volcanic environments (80-120°C, thick atmosphere, minimal water)
- **Cryophyte Crystals**: Silicon-based beings thriving in extreme cold (-50 to -10°C) and high radiation
- **Aquatic Molluscoids**: Ocean-dwelling species requiring extensive water coverage (70%+) and humid atmospheres
- **Desert Nomads**: Arid-adapted species preferring hot, dry conditions with minimal water
- **High-Gravity Hunters**: Powerful beings requiring dense worlds with 2-3x Earth gravity
- **Energy Feeders**: Radiation-consuming entities that thrive in high-energy environments

### Terraforming Tools
- **Temperature Control**: Heat generators and cooling systems
- **Atmospheric Processing**: Gas composition modification and pressure control
- **Water Management**: Ice delivery and ocean formation technologies
- **Gravity Modification**: Planetary mass manipulation
- **Radiation Shielding**: Magnetic field generation and radiation dampening

### Game Mechanics
- **Dynamic Market**: Alien species have limited availability and specific requirements
- **Progressive Unlocking**: Research and purchase advanced terraforming tools
- **Profit Optimization**: Maximize compatibility for higher sale prices (perfect fit: 4,000–6,000₵)
- **Strategic Planning**: Balance immediate costs against long-term investments
- **Visual Feedback**: Real-time planetary visualization and animated notifications

## 🛠️ Technology Stack

### Backend
- **PHP 8+** with Slim Framework
- **Doctrine ORM** for database management
- **MySQL/SQLite** database support
- **JWT Authentication** via WebHatchery Auth Portal
- **Composer** for dependency management

### Frontend
- **React 19** with TypeScript
- **Vite** for build tooling and development server
- **Tailwind CSS** for styling
- **Zustand** for state management
- **Framer Motion** for animations
- **React Router** for navigation
- **MSW** for API mocking in development

### Development Tools
- **ESLint** for code linting
- **TypeScript** for type safety
- **Chart.js** for data visualization
- **React Use** for utility hooks

## 🚀 Quick Start

### Prerequisites
- **PHP 8.1+** with Composer
- **Node.js 18+** with npm
- **MySQL 8.0+** or **SQLite 3+**
- **Git** for version control

### 1. Clone the Repository
```bash
git clone <repository-url>
cd planet_trader
```

### 2. Backend Setup
```bash
# Navigate to backend directory
cd backend

# Install PHP dependencies
composer install

# Configure database (see backend/SETUP_INSTRUCTIONS.md)
# Update config/database.php with your database credentials

# Run database migration and seeding
php scripts/migrate.php up
```

### 3. Frontend Setup
```bash
# Navigate to frontend directory
cd ../frontend

# Install Node.js dependencies
npm install

# Start development server
npm run dev
```

### 4. Access the Application
- **Frontend**: http://localhost:5173 (Vite dev server)
- **Backend API**: http://localhost:3001 (configure in backend)

## 📁 Project Structure

```
planet_trader/
├── backend/                 # PHP Slim API backend
│   ├── config/             # Database and app configuration
│   ├── public/             # Web server entry point
│   ├── scripts/            # Database setup and utility scripts
│   ├── src/                # Source code
│   │   ├── Actions/       # Business logic layer
│   │   ├── Controllers/   # HTTP request handlers
│   │   ├── Database/      # Database connection services
│   │   ├── Models/        # Data models
│   │   ├── Repositories/  # Data access layer
│   │   ├── Routes/        # API route definitions
│   │   └── Services/      # Business services
│   ├── storage/           # Logs and file storage
│   ├── tests/             # Backend tests
│   └── vendor/            # Composer dependencies
├── frontend/               # React TypeScript frontend
│   ├── public/            # Static assets
│   ├── src/               # Source code
│   │   ├── api/          # API integration layer
│   │   ├── components/   # React components
│   │   ├── contexts/     # React context providers
│   │   ├── game/         # Game-specific components
│   │   ├── mocks/        # Mock data for development
│   │   ├── pages/        # Page components
│   │   ├── services/     # Business logic services
│   │   ├── styles/       # CSS and styling
│   │   ├── types/        # TypeScript type definitions
│   │   └── utils/        # Utility functions
│   └── dist/              # Build output
├── data/                  # Game data files
│   ├── alien_species.json         # Alien species definitions
│   ├── alien_species_types.json   # Species type classifications
│   ├── planet_names.json          # Planet name pool
│   ├── planet_types.json          # Planet type definitions
│   ├── terraforming_tools.json    # Tool definitions
│   └── tool_research.json         # Research progression
└── publish.ps1           # Deployment script
```

## 🔧 Configuration

### Backend Configuration
- **Database**: Configure in `backend/config/database.php`
- **Authentication**: Set up WebHatchery Auth Portal credentials in `.env`
- **CORS**: Configure allowed origins for frontend integration

### Frontend Configuration
- **API Base URL**: Update in `frontend/src/api/config.ts`
- **Development**: MSW mock server runs automatically in development mode

## 🎯 Gameplay

1. **Start Game**: Begin with limited credits and basic terraforming tools
2. **Buy Planets**: Purchase available planets from the market
3. **Analyze Conditions**: Examine planetary stats (temperature, atmosphere, water, gravity, radiation)
4. **Terraform**: Apply tools to modify planetary environments to match alien requirements
5. **Sell Planets**: Sell terraformed planets to compatible alien buyers for profit
6. **Research**: Unlock advanced tools and expand your terraforming capabilities
7. **Scale Business**: Use profits to purchase better equipment and access premium planets

## 🔐 Authentication

The application integrates with the **WebHatchery Auth Portal** for centralized authentication:

- JWT-based authentication
- Single sign-on (SSO) support
- Role-based access control
- Secure token management

## 🧪 Testing

### Backend Testing
```bash
cd backend
composer test
```

### Frontend Testing
```bash
cd frontend
npm run lint
```

## 📚 API Documentation

The backend provides a RESTful API with endpoints for:
- Game session management
- Planet purchasing and management
- Terraforming actions
- Market data and alien species
- Player inventory and statistics

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests if applicable
5. Submit a pull request

## 📄 License

This project is licensed under the MIT License - see the LICENSE file for details.

## 🙏 Acknowledgments

- Built with scientific principles of planetary science and astrobiology
- Inspired by real terraforming concepts and proposals
- Features realistic environmental preferences for diverse alien species
- Educational value in teaching planetary habitability and environmental engineering

---

**Ready to transform planets and build your terraforming empire!** 🚀🪐</content>
<parameter name="filePath">h:\WebHatchery\game_apps\planet_trader\README.md
