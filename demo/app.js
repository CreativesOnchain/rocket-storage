// State & Data
const KEY_REGISTRY = {
  "0x6847c4f92710ff8dd03022f7ad712ca46d7d35a8cc1bdfa41f352f336b618412": {
    path: "contract.exists.rocketMegapoolDelegate",
    solidity: 'keccak256(abi.encodePacked("contract.exists", 0xdF9b5...))',
    category: "Contract Registration (v1.4 Added)"
  },
  "0x11b1f53204d03a3a08dc0a52f30bb09ab26aeb2b07023ce8b7b8aef1c1fa3e9f": {
    path: "contract.exists.rocketVault",
    solidity: 'keccak256(abi.encodePacked("contract.exists", 0x3bDC...))',
    category: "Core Vault Security"
  },
  "0xd411a0c44c5ff0881944883f364e7c376f9bf285f1c9d81d22223c72b220377c": {
    path: "dao.protocol.setting.deposit.express.queue.rate",
    solidity: 'keccak256(abi.encodePacked(settingNamespace, "express.queue.rate"))',
    category: "Protocol Deposit Settings (RPIP-75)"
  },
  "0x401e4a2d7f99166f212726359eb31c19b0151121d5c7f89772bf62fc4856f663": {
    path: "network.revenue.node.share",
    solidity: 'keccak256(abi.encodePacked("network.revenue.node.share"))',
    category: "Universal Adjustable Revenue Split (RPIP-46)"
  },
  "0x38416629ae7d9d6d376bece85f29ddf8a9e40344400cb571a0ca95eeeb72049d": {
    path: "megapool.delegate.set",
    solidity: 'keccak256(abi.encodePacked("megapool.delegate.set")) (Metadata slot: head=0, tail=1)',
    category: "Megapool Delegate Deque Invariant"
  }
};

const ADVERSARIAL_TESTS = [
  {
    id: "undeclared-write",
    name: "Undeclared Storage Write",
    file: "tests/adversarial/undeclared-write.json",
    outcome: "FAIL(UndeclaredMutation)",
    status: "fail",
    desc: "An unauthorized mutation is injected into RocketStorage that is not in the reviewed manifest.",
    rejection: 'Verdict::Fail { reasons: [FailReason::UndeclaredMutation { key: "0xdeadbeef...", op: SetUint }] }'
  },
  {
    id: "swapped-addresses",
    name: "Swapped Contract Addresses",
    file: "tests/adversarial/swapped-addresses.json",
    outcome: "FAIL(WrongNewValue)",
    status: "fail",
    desc: "Two contracts have their destination addresses transposed. Triggers Phase 5 pairwise transposition detection.",
    rejection: 'Verdict::Fail { reasons: [FailReason::WrongNewValue { key: "contract.address.rocketVault", expected: "0x3bDC...", actual: "0x1d8f..." }] }'
  },
  {
    id: "wrong-uint",
    name: "Wrong Setting Uint Value",
    file: "tests/adversarial/wrong-uint.json",
    outcome: "FAIL(WrongNewValue)",
    status: "fail",
    desc: "Governance parameter sets an unapproved commission or quorum value differing from the reviewed RPIP.",
    rejection: 'Verdict::Fail { reasons: [FailReason::WrongNewValue { key: "network.node.commission.share", expected: "50000000000000000", actual: "90000000000000000" }] }'
  },
  {
    id: "missing-deletion",
    name: "Omitted Required Deletion",
    file: "tests/adversarial/missing-deletion.json",
    outcome: "FAIL(MissingRequiredEffect)",
    status: "fail",
    desc: "A decommissioned contract is not deregistered, leaving stale permissions active in storage.",
    rejection: 'Verdict::Fail { reasons: [FailReason::MissingRequiredEffect { key: "contract.name.0xOldContract...", op: DeleteString }] }'
  },
  {
    id: "duplicate-mutation",
    name: "Duplicate Mutator Call",
    file: "tests/adversarial/duplicate-mutation.json",
    outcome: "FAIL(MultiplicityMismatch)",
    status: "fail",
    desc: "A key is mutated multiple times within the upgrade transaction when only one write was declared.",
    rejection: 'Verdict::Fail { reasons: [FailReason::MultiplicityMismatch { key: "contract.address.rocketMegapoolFactory", expected: 1, actual: 2 }] }'
  },
  {
    id: "unexpected-call",
    name: "Unauthorized External Call",
    file: "tests/adversarial/unexpected-external-call.json",
    outcome: "FAIL(UnexpectedExternalCall)",
    status: "fail",
    desc: "Upgrade contract invokes an external target address outside the 4 reviewed initialisation calls.",
    rejection: 'Verdict::Fail { reasons: [FailReason::UnexpectedExternalCall { target: "0xMaliciousContract...", selector: "0x12345678" }] }'
  },
  {
    id: "type-drift",
    name: "Type Drift Mismatch",
    file: "tests/adversarial/type-drift.json",
    outcome: "FAIL(OpMismatch)",
    status: "fail",
    desc: "Mutator method invoked with incorrect type (e.g., setBytes32 instead of setAddress).",
    rejection: 'Verdict::Fail { reasons: [FailReason::OpMismatch { key: "contract.address...", expected_op: SetAddress, observed_op: SetBytes32 }] }'
  },
  {
    id: "wrong-chain-id",
    name: "Wrong Chain ID Validation",
    file: "tests/adversarial/wrong-chain-id.json",
    outcome: "FAIL(WrongChainId)",
    status: "fail",
    desc: "Fixture trace recorded on Sepolia (11155111) tested against Mainnet (1) manifest requirement.",
    rejection: 'Verdict::Fail { reasons: [FailReason::WrongChainId { expected: 1, actual: 11155111 }] }'
  },
  {
    id: "undecodable-key",
    name: "Undecodable Storage Key",
    file: "tests/adversarial/undecodable-key.json",
    outcome: "UNKNOWN(UndecodableKey)",
    status: "fail",
    desc: "Encountered a key not present in the KeyCatalogue. Fails closed with UNKNOWN (exit code 2).",
    rejection: 'Verdict::Unknown { reasons: [UnknownReason::UndecodableKey { raw_key: "0x37a9f4c..." }] }'
  },
  {
    id: "reordered-manifest",
    name: "Reordered Manifest Declaration",
    file: "tests/adversarial/reordered-manifest.json",
    outcome: "PASS (Set Reconciled)",
    status: "pass",
    desc: "Effects declared in a different order than execution. Successfully reconciled via set matching.",
    rejection: 'Verdict::Pass (All 234 effects and 4 external calls match set semantics)'
  }
];

const PROOF_DATA = {
  md: `# Rocket Storage Gate (rsg) — Attestation Report

## Protocol Upgrade: Rocket Pool v1.4 / Saturn 1
- **Network:** Ethereum Mainnet (Chain ID: 1)
- **Pre-upgrade Block:** 24,479,993
- **Execution Block:** 24,479,994
- **Upgrade Transaction:** \`0x2fc10aad3c1b00bdfa9b6fddab79e0f2688609848f8f7a1a6449ab42da38530c\`
- **RocketStorage Contract:** \`0x1d8f8f00cfa6758d7bE78336684788Fb0ee0Fa46\`
- **Upgrade Contract:** \`0x5b3B5C76391662e56d0ff72F31B89C409316c8Ba\`
- **Source Commit:** \`fb7d9c428dc3dddc3fbd3e634e3cb365655df89e\`

---

## 🟢 VERDICT: PASS

All observed RocketStorage mutations and external calls match the reviewed manifest.
- **Observed Storage Effects:** 234
- **Authorized External Calls:** 4
- **Exit Code:** 0

### Cryptographic Digest Bindings:
- **Observed Trace SHA-256:** \`6212d49dad9cab7c60132bcdfef43bc0a24629b0b61fef0d5ffa55dd6705c757\`
- **Manifest SHA-256:** \`40e13752aca6517d61af48a66fe4567ab246c4fa3b23dbd8a3351d1ab3456063\`
- **Review Record Signoff:** Dual independent signoff verified (oDAO & Protocol Security Council)`,

  json: `{
  "version": "1",
  "generated_at": "2026-09-05T17:03:30Z",
  "hashes": {
    "observed_trace_sha256": "6212d49dad9cab7c60132bcdfef43bc0a24629b0b61fef0d5ffa55dd6705c757",
    "manifest_sha256": "40e13752aca6517d61af48a66fe4567ab246c4fa3b23dbd8a3351d1ab3456063",
    "review_record_sha256": "c85d7b539fa3486be78a24aa70f5e3df74d9e0337856b3e7bc8c9cb6b69b5c23"
  },
  "pinned": {
    "chain_id": 1,
    "pre_block": 24479993,
    "pre_block_hash": "0x8c26a07a5b2c987c58afd5e3458115a5ce49e6e6e9cff72780d2dfa7b89f3bfd",
    "upgrade_tx": "0x2fc10aad3c1b00bdfa9b6fddab79e0f2688609848f8f7a1a6449ab42da38530c",
    "exec_block": 24479994,
    "upgrade_contract": "0x5b3B5C76391662e56d0ff72F31B89C409316c8Ba",
    "rocket_storage": "0x1d8f8f00cfa6758d7bE78336684788Fb0ee0Fa46",
    "source_commit": "fb7d9c428dc3dddc3fbd3e634e3cb365655df89e"
  },
  "verdict": {
    "status": "PASS",
    "matched_effects": 234,
    "matched_calls": 4
  }
}`,

  lock: `40e13752aca6517d61af48a66fe4567ab246c4fa3b23dbd8a3351d1ab3456063  manifest.yaml`
};

// TAB SWITCHING
function switchTab(tabId) {
  document.querySelectorAll('.tab-btn').forEach(b => b.classList.remove('active'));
  document.querySelectorAll('.tab-panel').forEach(p => p.classList.remove('active'));

  const btn = document.getElementById(`tab-${tabId}-btn`);
  const panel = document.getElementById(`tab-${tabId}`);
  if (btn) btn.classList.add('active');
  if (panel) panel.classList.add('active');
}

// TERMINAL SIMULATOR
let isExecuting = false;

function clearTerminal() {
  const term = document.getElementById('term-output');
  term.innerHTML = `
    <div class="term-line prompt-line">
      <span class="term-prompt">user@ethereum:~/rocket-storage$</span> <span class="term-text">clear</span>
    </div>
  `;
}

function appendTermLine(html, className = '') {
  const term = document.getElementById('term-output');
  const line = document.createElement('div');
  line.className = `term-line ${className}`;
  line.innerHTML = html;
  term.appendChild(line);
  term.scrollTop = term.scrollHeight;
}

async function executeCommand(cmdText) {
  if (isExecuting) return;
  isExecuting = true;

  switchTab('terminal');
  appendTermLine(`<span class="term-prompt">user@ethereum:~/rocket-storage$</span> <span class="term-text">${cmdText}</span>`);

  if (cmdText.includes('validate-fixture')) {
    await sleep(250);
    appendTermLine('Fixture validation:');
    appendTermLine('  chain_id:         1');
    appendTermLine('  pre_block:        24479993');
    appendTermLine('  pre_block_hash:   0x8c26a07a5b2c987c58afd5e3458115a5ce49e6e6e9cff72780d2dfa7b89f3bfd');
    appendTermLine('  upgrade_tx:       0x2fc10aad3c1b00bdfa9b6fddab79e0f2688609848f8f7a1a6449ab42da38530c');
    appendTermLine('  exec_block:       24479994');
    appendTermLine('  upgrade_contract: 0x5b3b5c76391662e56d0ff72f31b89c409316c8ba');
    appendTermLine('  rocket_storage:   0x1d8f8f00cfa6758d7be78336684788fb0ee0fa46');
    appendTermLine('  source_commit:    fb7d9c428dc3dddc3fbd3e634e3cb365655df89e');
    appendTermLine('  ✅ All pinned parameters match.', 'output-pass');
  } else if (cmdText.includes('rsg attest')) {
    await sleep(200);
    appendTermLine('[rsg] Loading frozen trace from fixtures/v1.4-mainnet/frozen-trace.json…');
    await sleep(200);
    appendTermLine('[rsg] Loading manifest from manifests/v1.4-mainnet/manifest.yaml…');
    await sleep(300);
    appendTermLine('[rsg] Running comparator across 5 security verification phases…', 'output-cyan');
    await sleep(300);
    appendTermLine('[rsg] Proof bundle written to attestations/v1.4-mainnet');
    appendTermLine('[rsg] Observed-trace SHA-256: 6212d49dad9cab7c60132bcdfef43bc0a24629b0b61fef0d5ffa55dd6705c757');
    appendTermLine('[rsg] Manifest SHA-256: 40e13752aca6517d61af48a66fe4567ab246c4fa3b23dbd8a3351d1ab3456063');
    appendTermLine('<br><strong class="output-pass">✅  PASS</strong>');
    appendTermLine('    All 234 effects match the manifest.');
    appendTermLine('    All 4 external calls authorized.');
    appendTermLine('Proof bundle: attestations/v1.4-mainnet');
    appendTermLine('Exit code:    0', 'output-pass');
  } else if (cmdText.includes('decode-key')) {
    await sleep(150);
    appendTermLine('Key:           0x6847c4f92710ff8dd03022f7ad712ca46d7d35a8cc1bdfa41f352f336b618412');
    appendTermLine('Semantic path: contract.exists.rocketMegapoolDelegate', 'output-pass');
    appendTermLine('Solidity:      keccak256(abi.encodePacked("contract.exists", 0xdF9b5...))');
    appendTermLine('Exit code:     0');
  } else if (cmdText.includes('undeclared-write')) {
    await sleep(200);
    appendTermLine('[rsg] Loading adversarial trace: undeclared-write.json…');
    appendTermLine('[rsg] Running comparator…');
    appendTermLine('<br><strong class="output-fail">❌  FAIL (1 reason(s))</strong>');
    appendTermLine('    • {"UndeclaredMutation":{"key":"0xdeadbeef","op":"SetUint"}}', 'output-fail');
    appendTermLine('Exit code:    1', 'output-fail');
  } else if (cmdText.includes('swapped-addresses')) {
    await sleep(200);
    appendTermLine('[rsg] Loading adversarial trace: swapped-addresses.json…');
    appendTermLine('[rsg] Phase 5: Detecting pairwise address transpositions…', 'output-warn');
    appendTermLine('<br><strong class="output-fail">❌  FAIL (2 reason(s))</strong>');
    appendTermLine('    • {"WrongNewValue":{"key":"contract.address.rocketVault","expected":"0x3bDC...","actual":"0x1d8f..."}}', 'output-fail');
    appendTermLine('    • {"WrongNewValue":{"key":"contract.address.rocketMegapoolManager","expected":"0x1d8f...","actual":"0x3bDC..."}}', 'output-fail');
    appendTermLine('Exit code:    1', 'output-fail');
  } else if (cmdText.includes('--help')) {
    appendTermLine(`Rocket Pool RocketStorage Upgrade Effects Gate

Deterministically replays a Rocket Pool upgrade transaction and verifies that the RocketStorage mutations match a reviewed manifest.

Usage: rsg <COMMAND>

Commands:
  capture          Capture the live upgrade trace from an archive RPC
  attest           Compare a frozen trace against the manifest and produce a proof bundle
  hash-manifest    Print the SHA-256 hash of a manifest file
  validate-fixture Validate pinned fixture parameters (chain ID, block hash, tx hash)
  decode-key       Decode a raw bytes32 RocketStorage key into its semantic path
  help             Print this message or the help of the given subcommand(s)

Exit codes: 0=PASS  1=FAIL  2=UNKNOWN  3=tool-error`);
  }

  isExecuting = false;
}

function sleep(ms) {
  return new Promise(resolve => setTimeout(resolve, ms));
}

// FULL PIPELINE SIMULATION
async function runFullPipeline() {
  const nodes = [0, 1, 2, 3, 4];
  const btn = document.getElementById('run-pipeline-btn');
  btn.disabled = true;
  btn.innerHTML = '<span class="btn-icon">⏳</span> Replaying Transaction...';

  for (let i of nodes) {
    const node = document.getElementById(`p-node-${i}`);
    node.classList.add('active');
    await sleep(350);
  }

  btn.disabled = false;
  btn.innerHTML = '<span class="btn-icon">⚡</span> Replay & Run Full Verification';
}

// KEY DECODER
function setAndDecode(rawKey) {
  document.getElementById('key-input').value = rawKey;
  handleManualDecode();
}

function handleManualDecode() {
  const input = document.getElementById('key-input').value.trim();
  const box = document.getElementById('decode-result');
  const data = KEY_REGISTRY[input];

  if (data) {
    box.innerHTML = `
      <div class="result-header">
        <span class="status-badge valid">DECODED SUCCESSFULLY</span>
      </div>
      <div class="result-grid">
        <div class="r-item">
          <span class="r-k">Raw Key:</span>
          <code class="r-v text-cyan">${input}</code>
        </div>
        <div class="r-item">
          <span class="r-k">Semantic Path:</span>
          <code class="r-v text-green" style="font-weight:700;">${data.path}</code>
        </div>
        <div class="r-item">
          <span class="r-k">Solidity Origin:</span>
          <code class="r-v text-orange">${data.solidity}</code>
        </div>
        <div class="r-item">
          <span class="r-k">Category:</span>
          <span class="r-v">${data.category}</span>
        </div>
      </div>
    `;
  } else {
    box.innerHTML = `
      <div class="result-header">
        <span class="status-badge unknown">UNDECODABLE KEY (FAILS CLOSED)</span>
      </div>
      <div class="result-grid">
        <div class="r-item">
          <span class="r-k">Raw Key:</span>
          <code class="r-v text-pink">${input}</code>
        </div>
        <div class="r-item">
          <span class="r-k">Verdict:</span>
          <code class="r-v text-pink">UNKNOWN (Exit code 2)</code>
        </div>
        <div class="r-item">
          <span class="r-k">Security Action:</span>
          <span class="r-v">Halts pipeline immediately to prevent unauthorized or ambiguous storage mutations.</span>
        </div>
      </div>
    `;
  }
}

// ADVERSARIAL MATRIX INITIALIZATION
function initAdversarialCards() {
  const container = document.getElementById('adv-cards');
  container.innerHTML = '';

  ADVERSARIAL_TESTS.forEach((test, idx) => {
    const card = document.createElement('div');
    card.className = `adv-card ${test.status === 'pass' ? 'pass-scenario' : ''}`;
    card.id = `adv-card-${test.id}`;
    card.onclick = () => selectAdversarial(test);

    card.innerHTML = `
      <div class="adv-title">${test.name}</div>
      <div class="adv-outcome ${test.status === 'pass' ? 'text-green' : 'text-pink'}">${test.outcome}</div>
    `;
    container.appendChild(card);
  });
}

function selectAdversarial(test) {
  document.querySelectorAll('.adv-card').forEach(c => c.classList.remove('selected'));
  const card = document.getElementById(`adv-card-${test.id}`);
  if (card) card.classList.add('selected');

  document.getElementById('adv-detail-title').innerText = `${test.name} (${test.file})`;
  const tag = document.getElementById('adv-detail-tag');
  tag.innerText = test.outcome;
  tag.className = `tag ${test.status === 'pass' ? 'tag-green' : 'tag-pink'}`;

  document.getElementById('adv-detail-content').innerHTML = `
    <div style="margin-bottom:0.75rem;"><strong>Attack Hypothesis:</strong> ${test.desc}</div>
    <div style="margin-bottom:0.5rem; color:#9ca3af;"><strong>Comparator Evaluation & Rejection:</strong></div>
    <div style="background:rgba(0,0,0,0.4); padding:0.75rem; border-radius:6px; border:1px solid rgba(255,255,255,0.06); color:${test.status === 'pass' ? '#10b981' : '#f43f5e'};">
      ${test.rejection}
    </div>
  `;
}

// PROOF BUNDLE VIEWER
function showProofFile(type) {
  document.querySelectorAll('.file-tab').forEach(t => t.classList.remove('active'));
  document.getElementById(`btn-show-${type}`).classList.add('active');

  const content = PROOF_DATA[type] || '';
  document.getElementById('proof-content').innerText = content;
}

// INIT
window.addEventListener('DOMContentLoaded', () => {
  initAdversarialCards();
  showProofFile('md');
  if (ADVERSARIAL_TESTS.length > 0) {
    selectAdversarial(ADVERSARIAL_TESTS[0]);
  }
});
