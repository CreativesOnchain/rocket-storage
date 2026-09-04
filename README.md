# rocket-storage-gate (`rsg`)

> **Deterministic Rocket Pool RocketStorage Upgrade Verification Gate**

`rsg` is a standalone, open-source CLI tool and library ecosystem that deterministically replays Rocket Pool protocol upgrades and verifies whether the upgrade transaction produced **exactly** the declared `RocketStorage` mutations and external calls—with **no undeclared side effects**.

Targeting the executed **Rocket Pool v1.4 / Saturn 1 mainnet upgrade**, `rsg` compares observed on-chain state mutations against a reviewed, source-anchored expected-effects manifest and generates a cryptographic proof bundle with reproducible `PASS`, `FAIL`, or `UNKNOWN` attestations.

---

## The Problem

Standard smart contract verification confirms that published source code, bytecode, and deployed contract addresses match. However, it leaves a critical gap:

> *Did the upgrade transaction make every expected typed RocketStorage change—and absolutely no undeclared or malicious changes?*

`RocketStorage` uses opaque `bytes32` keys and generic typed mutators (`setUint`, `setAddress`, `setBool`, `deleteAddress`, `addUint`, etc.). Raw transaction traces display only obscure hashes and hex values. `rsg` adds the missing **semantic decoding layer**, translating raw keys into human-auditable paths and strictly verifying them against an independently authored and reviewed manifest.

---

## Workspace Architecture

```
rocket_storage/
├── Cargo.toml
├── rust-toolchain.toml
├── key-catalogue.yaml                  # Canonical catalogue of RocketStorage keys
├── crates/
│   ├── rsg-types/                     # Shared data model, serialization, and error types
│   ├── rsg-decode/                    # Keccak256 key decoder & semantic dictionary
│   ├── rsg-capture/                   # Live archive RPC tracer (alloy 2.x callTracer)
│   ├── rsg-compare/                   # Strict verdict engine (PASS / FAIL / UNKNOWN)
│   ├── rsg-attest/                    # Proof bundle generator (JSON + Markdown reports)
│   └── rsg-cli/                       # Command-line interface (`rsg`)
├── fixtures/v1.4-mainnet/
│   └── frozen-trace.json              # Canonical frozen upgrade trace
├── manifests/v1.4-mainnet/
│   ├── manifest.yaml                  # Source-anchored expected effects manifest
│   └── review-record.json             # Dual-signoff reviewer record
└── tests/
    └── adversarial/                   # 11 adversarial test fixtures testing all failure modes
```

---

## Pinned Saturn 1 Parameters

| Parameter | Value |
|---|---|
| **Protocol Upgrade** | Rocket Pool v1.4 / Saturn 1 |
| **Network** | Ethereum Mainnet (Chain ID: `1`) |
| **Pre-upgrade Block** | `24,479,993` |
| **Execution Block** | `24,479,994` |
| **Upgrade Contract** | `0x5b3B5C76391662e56d0ff72F31B89C409316c8Ba` |
| **Upgrade Transaction** | `0x2fc10aad3c1b00bdfa9b6fddab79e0f2688609848f8f7a1a6449ab42da38530c` |
| **RocketStorage** | `0x1d8f8f00cfa6758d7bE78336684788Fb0ee0Fa46` |
| **Source Commit** | `fb7d9c428dc3dddc3fbd3e634e3cb365655df89e` |

---

## Installation & Building

Requirements: Rust toolchain (2024 edition, Rust 1.85+).

```bash
# Build the workspace
cargo build --release

# Run all unit and integration tests
cargo test
```

Binary is output at `target/release/rsg`.

---

## CLI Usage

### 1. Attest (Offline Verification)

Verify an upgrade trace against the reviewed manifest without network access:

```bash
rsg attest \
  --fixture fixtures/v1.4-mainnet/frozen-trace.json \
  --manifest manifests/v1.4-mainnet/manifest.yaml \
  --review-record manifests/v1.4-mainnet/review-record.json \
  --output-dir attestations/v1.4-mainnet/
```

### 2. Live Capture & Attest (Online Verification)

Capture the upgrade trace directly from an Ethereum archive node RPC:

```bash
rsg capture \
  --rpc-url https://eth-mainnet.g.alchemy.com/v2/YOUR_KEY \
  --output fixtures/v1.4-mainnet/frozen-trace.json
```

Or combine capture and attestation in a single run:

```bash
rsg attest \
  --rpc-url https://eth-mainnet.g.alchemy.com/v2/YOUR_KEY \
  --manifest manifests/v1.4-mainnet/manifest.yaml \
  --review-record manifests/v1.4-mainnet/review-record.json \
  --output-dir attestations/v1.4-mainnet/
```

### 3. Decode Raw Keys

Decode an arbitrary `bytes32` RocketStorage key into its human-readable semantic path:

```bash
rsg decode-key 0x11b1f53204d03a3a08dc0a52f30bb09ab26aeb2b07023ce8b7b8aef1c1fa3e9f
```

---

## Verdicts & Exit Codes

`rsg` implements fail-closed verification designed for automated CI/CD pipelines:

| Verdict | Exit Code | Description |
|---|:---:|---|
| **`PASS`** | `0` | All observed mutations and external calls match the reviewed manifest exactly. |
| **`FAIL`** | `1` | A known requirement is violated (e.g., undeclared write, wrong value, duplicate mutation, unexpected call). |
| **`UNKNOWN`** | `2` | An undecodable key, unsupported call format, or incomplete trace was encountered. Fails closed. |

---

## Adversarial Test Suite

The test suite includes 11 dedicated adversarial test cases validating that every failure mode halts with the exact expected verdict:

1. **`undeclared-write.json`**: An unmanifested mutation occurs &rarr; `FAIL(UndeclaredMutation)`
2. **`wrong-uint.json`**: New value differs from declared &rarr; `FAIL(WrongNewValue)`
3. **`swapped-address.json`**: Address points to incorrect contract &rarr; `FAIL(WrongNewValue)`
4. **`missing-deletion.json`**: Required deletion was not executed &rarr; `FAIL(MissingRequiredEffect)`
5. **`duplicate-mutation.json`**: Multiple writes to the same key &rarr; `FAIL(MultiplicityMismatch)`
6. **`wrong-multiplicity.json`**: Effect count does not match requirement &rarr; `FAIL(MultiplicityMismatch)`
7. **`wrong-selector.json`**: External call with unauthorized selector &rarr; `FAIL(UnexpectedExternalCall)`
8. **`unexpected-initializer.json`**: External call to non-allowed contract &rarr; `FAIL(UnexpectedExternalCall)`
9. **`type-drift.json`**: Mutator called with wrong type method &rarr; `FAIL(OpMismatch)`
10. **`wrong-chain.json`**: Fixture executed on wrong chain ID &rarr; `FAIL(WrongChainId)`
11. **`reordered-manifest.json`**: Different manifest declaration order &rarr; `PASS` (set-based reconciliation)

---

## Proof Bundle Outputs

Running `rsg attest` produces deterministic artifacts in the output directory:

```
attestations/v1.4-mainnet/
├── attestation.json       # Deterministic JSON attestation with SHA-256 bindings
├── attestation.md         # Reviewer-readable Markdown report
├── observed-trace.json    # Canonical normalized trace
├── manifest.lock          # SHA-256 lockfile of the manifest
└── review-record.json     # Signed record of independent review
```

---

## Observation Boundary & Non-Goals

- **Observation Boundary**: `rsg` inspects calls to the `RocketStorage` contract and immediate outbound calls originating from the upgrade contract. Internal storage changes within other contracts (e.g., proxy implementation slots) are outside this boundary.
- **Non-Goals**:
  - `rsg` does not replace Foundry or Anvil.
  - `rsg` is not a smart contract audit and does not claim that the upgrade logic is safe.
  - `rsg` does not automatically infer governance intent from text RPIPs.
