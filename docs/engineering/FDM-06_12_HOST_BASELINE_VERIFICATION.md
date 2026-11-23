# FDM-06.12 — Host Baseline Verification Trigger

This documentation-only change exists to obtain a reproducible CI baseline for the exact AniCraft source tree audited by FDM-06.10 and FDM-06.11.

- Repository: `Mujshanan/anicraft`
- Audited base: `main` at `2df7775907d1458ce653e7e6314ec8a5d7d565f5`
- Base tree: `80071fb3f714944e49608668871d4cf53a6159a5`
- Workflow: `.github/workflows/ci.yml`
- Trigger: pull request
- Scope of this change: documentation only

## Required baseline checks

The workflow delegates its quality gate to `storytold/craft-common-actions/.github/workflows/gate.yml@main` and supplies:

```sh
cargo test --workspace && cargo clippy --workspace --all-targets -- -D warnings
```

The independent licence job runs:

```sh
cargo deny --locked check licenses bans sources
```

The workflow also provisions the Linux packages required for GUI, software Vulkan parity, audio and media tests. Windows/macOS ffmpeg-dependent tests are expected to skip when ffmpeg is unavailable, as described by the workflow.

## Interpretation

This PR does not change Rust code, dependencies, workflow configuration, or release artifacts. Its CI result is a baseline for the pinned host source plus this documentation-only delta. A passing run is evidence that the host baseline compiles/tests under the workflow's configured environment; it is not evidence that the proposed FDM transaction API has been implemented or that the S0 migration gates are closed.

Record each job's exact conclusion, run URL, commit SHA, runner/platform and any skipped tests in the FDM-06.12 report. Do not repair unrelated baseline failures in this PR.
