# Planet Trader Phase 1 Direction

## Product diagnosis

The Rust port is a faithful, sturdier reproduction of the legacy React game:
both open on the same three-column dashboard, use the same one-card tutorial,
and expose the same buy-adjust-sell loop. Persistence, research, history,
salvage, and biosphere were added behind that shell, but the first minute still
reads as Phase 0 because those systems do not change the player's journey or
the game's visual identity.

Phase 1 treats Planet Trader as a company-building fantasy rather than a single
dashboard. The player should feel that they are running a frontier brokerage:
choosing contracts, shaping worlds in a workshop, reading a changing market,
and growing a company whose capabilities open new strategies.

## Retain, replace, add

Retain the typed data, deterministic rule helpers, save migration, planet/tool
simulation, buyer compatibility, trade ledger, research definitions, and the
Macroquad Toolkit integration. These are useful foundations rather than the
finished experience.

Replace the tutorial-first dashboard entry, flat three-column information
hierarchy, utility-button header, and modal-only progression. The dashboard may
remain temporarily available while its responsibilities move into focused
screens, but it is not the Phase 1 destination.

Add a branded home screen; functional settings; a persistent action-driven
tutorial; focused Acquisition, Workshop, Market, Research, and Company modes;
meaningful biosphere demand; attainable research; company reputation and
progression; clearer recovery; richer planet rendering; and state-specific
feedback.

## Delivery sequence

1. **Front door and onboarding.** Ship a home screen with New Company,
   Continue, Settings, save-aware summaries, and an action-driven tutorial.
   Settings use the toolkit's persistent display controls and expose only
   behavior that works.
2. **Navigation and visual language.** Replace the three-column shell with a
   persistent command deck and focused modes. Establish the Phase 1 palette,
   typography, planet hero treatment, transitions, and responsive hierarchy.
3. **Acquisition and workshop strategy.** Turn planet buying into contract
   evaluation; make tool effects, side effects, investment, target ranges, and
   recovery legible before committing credits.
4. **Market and biosphere economy.** Give species explicit biosphere demand,
   make every displayed stat affect compatibility, add market signals, and
   explain projected profit and RP before a sale.
5. **Company and research progression.** Tune RP awards/costs, add company
   reputation and milestones, make unlocks change strategies, and surface the
   ledger as progress rather than bookkeeping.
6. **Finish and prove.** Complete settings/about/save flows, polish transitions
   and feedback, migrate old saves, replace verification captures, and compare
   Phase 0/Phase 1 at 1280x720 and 1024x768.

Each step must end in a player-visible vertical slice that passes tests,
Clippy, `publish.ps1`, and interactive WebGL checks before it is committed and
pushed. Internal hardening supports a slice but is never the slice itself.

## Delivery status

- Front door and onboarding: shipped.
- Navigation and visual language: command-deck modes shipped and verified.
- Market/biosphere and company/research depth: six-axis demand, attainable RP,
  reputation ranks, and expanding contract reach shipped.
- Acquisition/workshop decision previews: live buyer routes, best-deal margins,
  six-stat workshop hierarchy, and disclosed salvage recovery shipped.
- Final finish/proof: shipped. The complete onboarding and first-sale journey
  was exercised in the published WebGL build at 1280x720, then Research and
  Company were checked at 1024x768. Phase 0/Phase 1 captures and the visible
  feature comparison are preserved in `docs/PHASE_1_COMPARISON.md`.
