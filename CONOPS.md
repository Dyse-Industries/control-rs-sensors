# Workspace Concept of Operations (CONOPS): `control-rs-sensors`

## 1. Overview & Purpose

`control-rs-sensors` develops data-driven sensor models for simulation within the `control-rs` ecosystem.

Developers use this workspace to:
- Identify sensor error models (noise, bias, scale factor, latency, quantization and saturation) from recorded hardware data.
- Expose identified models as deterministic measurement sources for `control-rs` simulations.
- Verify each model against held-out recordings of the sensor it represents.

---

## 2. Component Role & System Architecture

`control-rs-sensors` sits between recorded hardware data and simulated plants:

```
[Recorded Sensor Data]
  └── Bench / Flight Logs ───────────────┐
                                         ├──> [control-rs-sensors: Sensor Models]
[Simulated Plant State (control-rs)] ────┘      ├── Error Model Identification
                                                ├── Measurement Synthesis
                                                └── Model-vs-Data Verification
                                                          │
                                                          ▼
                                       [Simulated Measurements]
                                       (Estimators, Controllers, Layer 3 GNC)
```

Sensor classes, model structures and acceptance criteria are defined by the design document produced through the `cr-research` and `cr-design-doc` pipeline.

---

## 3. Workspace Layout & Artifact Contracts

```
control-rs-sensors/
├── Cargo.toml               # Package manifest (control-rs by git revision)
├── .cargo/
│   ├── config.toml          # Cargo aliases (lint, fmt-check, fmt-all)
│   └── gate.toml            # control-rs-ci gate configuration
├── .github/
│   └── workflows/ci.yml     # Automated CI verification workflow
├── src/
│   └── lib.rs               # Crate root
└── CONOPS.md                # Workspace operations and developer guide (this file)
```

---

## 4. Development & Verification Workflow

### 4.1 Standard Toolchain Commands

```bash
# Run every gate (fetch, build, no-std, test, fmt, clippy, doc)
control-rs-ci

# Run selected gates
control-rs-ci --only no-std,test

# Gate artifacts and ci-report.md
ls target/ci-artifacts
```

### 4.2 Model Verification & Invariants
When adding or modifying a sensor model:
1. **Data Provenance**: Every identified parameter traces to a named recording and identification procedure.
2. **Determinism**: Measurement synthesis is reproducible for a fixed seed and input trajectory.
3. **Zero Dynamic Allocation**: Model state and measurement synthesis remain strictly `#![no_std]` compatible.
