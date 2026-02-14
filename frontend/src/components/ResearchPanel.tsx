interface Research {
  id: string;
  name: string;
}

interface ResearchPanelProps {
  researchPoints: number;
  researchList: Research[];
  onResearch: (research: Research) => void;
  unlockedResearch?: string[];
}

const ResearchPanel: React.FC<ResearchPanelProps> = ({
  researchPoints,
  researchList,
  onResearch,
  unlockedResearch = [],
}) => (
  <section className="research-panel">
    <div className="card">
      <div className="card__header">
        <h3>🔬 Research Lab</h3>
        <div className="rp-total">
          Research Points: <span>{researchPoints}</span>
        </div>
      </div>
      <div className="card__body">
        <div className="research-list">
          {researchList.length === 0 ? (
            <div>No research available.</div>
          ) : (
            researchList.map(research => {
              const unlocked = unlockedResearch.includes(research.name);
              return (
                <div
                  key={research.id || research.name}
                  className={`research-item${unlocked ? ' unlocked' : ''}`}
                >
                  <span>{research.name}</span>
                  <button onClick={() => onResearch(research)} disabled={unlocked}>
                    {unlocked ? 'Unlocked' : 'Research (10 RP)'}
                  </button>
                </div>
              );
            })
          )}
        </div>
      </div>
    </div>
  </section>
);

export default ResearchPanel;
