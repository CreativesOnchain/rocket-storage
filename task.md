# rocket-storage-gate Build Tasks

## Phase 1: Workspace Foundation
- [x] `Cargo.toml` (workspace)
- [x] `rust-toolchain.toml`
- [x] `crates/rsg-types/` — shared data model
- [x] `crates/rsg-decode/` — semantic key decoder
- [x] `crates/rsg-capture/` — mutation capture engine
- [x] `crates/rsg-compare/` — verdict engine
- [x] `crates/rsg-attest/` — proof bundle generator
- [x] `crates/rsg-cli/` — CLI binary (`rsg`)

## Phase 2: Data Files
- [x] `key-catalogue.yaml`
- [x] `manifests/v1.4-mainnet/manifest.yaml`
- [x] `manifests/v1.4-mainnet/review-record.json`

## Phase 3: Run Capture + Build Golden Fixture
- [x] `cargo build` — compiles cleanly
- [x] `rsg capture --rpc-url <URL>` — live trace tested (tested against Alchemy RPC; pre-block validated)
- [x] `fixtures/v1.4-mainnet/frozen-trace.json` — golden fixture with exact pre-block hash

## Phase 4: Manifest + Attest
- [x] Manifest reconciliation & hashing (`rsg hash-manifest`)
- [x] `rsg attest` — proof bundle generation (JSON attestation + Markdown report + manifest.lock + review-record)

## Phase 5: Adversarial Tests + CI
- [x] `tests/adversarial/*.json` (11 adversarial fixtures covering all failure modes)
- [x] `crates/rsg-cli/tests/integration.rs` (13 integration tests passing)
- [x] `.github/workflows/ci.yml` — offline CI workflow
- [x] `README.md` — complete documentation
