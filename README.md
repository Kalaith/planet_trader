# Planet Trader

Planet Trader is the Rust + Macroquad port of the legacy Terraforming Co.
browser game. Buy randomized worlds, tune their environments with terraforming
tools, compare them with alien buyers, and sell a compatible planet for profit.

## Ported gameplay

- Touch-first tutorial and game entry flow.
- Randomized planet offers with planet types, names, stats, colors, and prices.
- Planet inventory with active-planet selection.
- Fifteen terraforming tools with costs, primary effects, side effects, and
  locked research requirements.
- Alien market with four rotating buyers, five-stat compatibility checks, buyer
  details, and compatibility-gated selling.
- Local autosave/load/reset using the macroquad-toolkit save slots.
- Embedded legacy data loaded through macroquad-toolkit::data_loader.

## Development

Run cargo fmt --all, cargo test, cargo clippy --all-targets --all-features
-- -D warnings, and then .\publish.ps1 from the project root.

The capture harness is .\scripts\capture_ui.ps1 -Scenes gameplay and writes
verification images to docs\verification\.
