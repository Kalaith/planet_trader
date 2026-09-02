# Knowledge and Precision UI Rebuild

This release implements the supplied gameplay and interface direction as one
integrated progression loop. Planet Trader remains a menu-driven brokerage game:
buy difficult worlds, learn what buyers value, terraform with imperfect tools,
and decide when another intervention is worth its cost.

## Implemented progression

- Credits fund contracts, research facilities, and terraforming operations.
- Universal RP purchases research; no species-specific spendable wallets were
  added.
- Five non-spendable knowledge fields grow through 4/6, 5/6, and 6/6 sales.
  Strong sales reveal previously unknown research branches.
- Research nodes support RP cost, credit cost, prerequisites, knowledge gates,
  tier, discovery hint, and a clear statement of what becomes visible or usable.
- Early tools use uncertain forecast ranges and Standard intensity. Planetary
  Systems Modeling unlocks Low and Heavy intensity; Precision Climate Grid
  provides exact fine climate adjustment.
- Market rotation pauses during orientation so a learner's selected buyer and
  requirements cannot disappear mid-step.

## Rebuilt decision screens

- **Acquire:** one large scanner, a compact live-demand pulse, and one primary
  scan action. The contract view uses an offer rail, a large selected planet,
  and a separate capital/route decision zone.
- **Workshop:** only researched tools appear. The planet dominates the central
  cradle; current values, predicted changes, confidence, cost, and Apply Tool
  are separated into an outcome inspector.
- **Alien Market:** one world summary, four compact offers, and one six-row deal
  analysis. The 4/6 threshold, payout, margin, RP, and knowledge reward are shown
  before Close Deal.
- **Research:** known branches are selectable; unknown branches are concise
  themed hints rather than a wall of padlocks. Nodes distinguish completed,
  affordable, prerequisite-blocked, and undiscovered states.
- **Company:** capital, profit, reputation, RP, ledger, and all five species
  knowledge tracks are visible without planet-specific engineering controls.

## Verification evidence

The current `docs/verification/` captures cover acquisition, a five-contract
scan, beginner workshop, exact advanced workshop, selected market deal,
research discovery, company knowledge, settings, and touch-first tutorial
states. `ui_advanced.png` specifically demonstrates the late-game precision
tool and exact outcome model.

The published WebGL build was exercised through visible controls from a new
company to contract scan, acquisition, portfolio selection, buyer analysis,
workshop intervention, a compatible sale, Research, and orientation completion.
