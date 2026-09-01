# Planet Trader

Planet Trader is the Rust + Macroquad port of the legacy Terraforming Co.
browser game. Buy randomized worlds, tune their environments with terraforming
tools, compare them with alien buyers, and sell a compatible planet for profit.

## Phase 1 gameplay

- Branded home screen with save-aware New Company and Continue flows.
- Focused command-deck navigation for Acquisition, Workshop, Alien Market,
  Research, and Company instead of one overloaded dashboard.
- Persistent, action-driven orientation that follows real player actions.
- Functional display/readability settings, tutorial restart, and save management.
- Randomized planet offers with planet types, names, stats, colors, and prices.
- Contract scans compare every unmodified world against the best live buyer
  route before purchase; Workshop shows the best current deal and exact
  salvage recovery before further investment.
- Planet inventory with active-planet selection.
- Fifteen terraforming tools with costs, primary effects, side effects, and
  locked research requirements.
- Alien market with a dedicated Deal Room, four rotating buyers, six-stat
  compatibility including biosphere demand, exact profit/RP previews, and
  compatibility-gated selling.
- Attainable research costs and company reputation ranks earned through strong
  sales; higher reputation expands each acquisition scan from three contracts
  to as many as five.
- Local autosave/load/reset using the macroquad-toolkit save slots.
- Embedded legacy data loaded through macroquad-toolkit::data_loader.

The staged Phase 1 direction and delivery sequence are documented in
`docs/PHASE_1_DESIGN.md`. The player-visible Phase 0/Phase 1 comparison is in
`docs/PHASE_1_COMPARISON.md`.

## Development

Run cargo fmt --all, cargo test, cargo clippy --all-targets --all-features
-- -D warnings, and then .\publish.ps1 from the project root.

The capture harness is .\scripts\capture_ui.ps1 -Scenes gameplay and writes
verification images to docs\verification\.
