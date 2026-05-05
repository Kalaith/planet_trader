import { useGameEffects } from './game/useGame';
import { GamePage } from './pages/GamePage';

function App() {
  useGameEffects();

  return <GamePage />;
}

export default App;
