# Munarium Sentinel

**Telemetry, anomaly detection, circuit breakers, incident replay.** Sentinel is the
assurance-plane component of the Munarium Governance Platform that provides runtime observation and
incident reconstruction from the platform's own records. It shows a task's sequence of proposals,
decisions, approvals, claims, dispatches, receipts and unresolved outcomes with the exact evidence
and policy versions observed at each step, and it can request a pre-authorized suspension through
Warden. It cannot rewrite the policy it monitors or grant an agent additional capability.

> **Status: Planned — Rust scaffold present.** This checkout contains a dependency-free,
> non-publishable [Cargo library](Cargo.toml) and documented interfaces under [src/](src/lib.rs).
> The interfaces have no implementations: no runtime service, client transport, database,
> provider integration or contract implementation is available. No production path is qualified.
> Build checks validate source structure, not governance capabilities. The
> [capability table](#capability-status) remains the authoritative functional status.

Sentinel is one of nine components built around the existing Munarium foundation, Munarium Server
and Munarium Matrix. Their shared architecture, normative contracts, decision records, roadmap and
composition evidence live in the public hub,
[iokaio/munarium-platform](https://github.com/iokaio/munarium-platform). This repository will hold
Sentinel's implementation, its unit and component tests, the migrations it owns, operational
diagnostics, package definitions, a local development recipe and release evidence. It is open
source from its first public commit, under the Apache License 2.0, with no proprietary edition.

## Start building

Read the [development index](docs/README.md), then the [architecture](docs/architecture.md),
[implementation plan](docs/implementation-plan.md) and [validation guide](docs/validation.md).
They map the public platform plan to source modules, dependencies, a first bounded work item
and acceptance cases. Runtime capabilities remain planned; supported contract versions are **none**.

## What Sentinel is for

The assurance plane reads authoritative records and may hold rebuildable views. Sentinel is the
operational half of that plane: what is happening now, what happened during an incident, and the
one bounded response it may request. **Local enforcement does not depend on Sentinel being
available.** Gate's authorization and Gateway's hard budget checks continue without a dashboard;
where an action explicitly requires current monitoring, that requirement is an admission obligation
that fails closed.

### On the name

Munarium Sentinel is a software assurance component. The platform plan acknowledges Jamey Kistner
of OSINTelligence LLC, whose *The Sovereign Stack* describes a hardware-isolated External Sentinel
with verification authority beyond the governed system's write reach. This component does not
claim equivalence to that design, and the acknowledgment does not imply endorsement. The projects
retain their own scope and implementation evidence.

## The design, as planned

### Useful before sophisticated

The first release shows a task's sequence of proposals, decisions, approvals, claims, dispatches,
receipts and unresolved outcomes, identifying the exact evidence and policy versions observed at
each decision. That is a record of the system's inputs and actions, **not access to the model's
internal beliefs**.

Basic counters come first: denial and escalation rates, grant issuance and rejection, action
frequency, missing receipts, budget saturation, dependency health and connector drift. A simple
alert on an impossible state transition is more valuable initially than a complex anomaly model
without a reliable underlying event contract.

OpenTelemetry export uses an **explicitly pinned convention version**. Generative-AI conventions
evolve, so export compatibility is versioned rather than assumed. Sensitive payloads are excluded or
redacted according to the deployment policy.

### Circuit breakers

A breaker request identifies the caller, scope, reason, triggering evidence and requested duration.
**Warden authenticates and enforces the suspension** according to a previously approved policy.
Sentinel may be permitted to narrow capability automatically; **it may not restore broader
authority** because an anomaly score fell. Restoration after a serious suspension follows the
applicable human or deterministic approval path.

An external SIEM or SOAR may submit the same bounded request. Neither a SIEM alert nor a telemetry
connector becomes a general governance administrator.

The initial implementation tests the **measured time from an authenticated suspension request to
rejection of new affected grants**, and shows what happens to already dispatched work and to
unconsumed grants. A dashboard that displays suspended while the action path still accepts work has
failed its most important contract.

### Blind spots and rebuildable views

Sentinel may maintain indexes and materialized views for performance, but their records retain
source identifiers and watermarks and can be rebuilt from the authoritative ledger and declared
event sources. **A missing interval is visible**, not silently filled with an assumption that
nothing happened. A comparison between registered capabilities and observed activity identifies
drift and uncovered paths, and the record also identifies discovery blind spots.

## First public increment

**A ledger-derived task timeline and an authenticated suspension request**, with the basic
counters, one impossible-transition alert, and the measured suspension bound for the reference
topology.

Target window: Stage 3 (months 7–9).

## Capability status

The labels are evidence labels, not editions: **Planned**, **Experimental**, **Conformance-tested**,
**Reference-qualified**, **Independently reviewed**. In the hub's component catalog this
repository is at **repository created**.

| Capability | Status | Evidence |
|---|---|---|
| Task timeline from the authoritative ledger, with evidence and policy versions per decision | Planned | none |
| Basic counters: denials, escalations, grants, action frequency, missing receipts, budget saturation, dependency health, connector drift | Planned | none |
| Impossible-state-transition alerts | Planned | none |
| OpenTelemetry export at a pinned convention version, with payload redaction | Planned | none |
| Authenticated circuit-breaker request through Warden under pre-authorized policy | Planned | none |
| Measured suspension propagation for one reference topology | Planned | none |
| Rebuildable views with source identifiers, watermarks and visible gaps | Planned | none |
| Registered-versus-observed drift comparison | Planned, later | none |
| SIEM and SOAR integrations submitting the same bounded request | Planned, later | none |
| Anomaly models | Deferred until the event contract is reliable | none |

Supported contract versions: **none**. Supported telemetry backends: **none**. Operations available
today: **none**.

## Acceptance evidence for the first release

| Test | Required outcome |
|---|---|
| Suspension propagation | New affected grants are refused within the measured, published bound; the fate of dispatched work and unconsumed grants matches the contract |
| Displayed state versus enforced state | Never shows suspended while the path accepts work |
| Breaker attempting to restore authority | Refused; restoration follows the approval path |
| Missing interval in a source | Rendered as a gap, with the watermark that bounds it |
| View rebuild | Reproduces the view from the ledger and declared event sources |
| Sentinel unavailable | Gate and Gateway continue to enforce; actions requiring current monitoring stop |
| Telemetry export | Conforms to the pinned convention version; sensitive payloads excluded or redacted per policy |
| SIEM-originated request | Treated as a bounded breaker request, not administration |

A blank evidence field means unverified, not passed.

## Invariants

| ID | Required property | Owner and first gate |
|---|---|---|
| INV-12 | Revoked authority stops new affected work within the qualified bound | Warden and Sentinel; stages 2–3 |
| INV-17 | Missing evidence or telemetry is reported as a gap, not a successful interval | Sentinel and Assure; stages 3–4 |
| INV-22 | A release advertises only the profiles and capabilities supported by its evidence | every component; every stage |

## Contracts, dependencies and neighbors

- **Contracts.** The hub's contracts directory is normative for the action-record shapes Sentinel
  reads, the circuit-breaker request and the pinned telemetry schema. Supported contract versions:
  none yet.
- **Foundation.** Munarium Server 1.3.0's ledger is the authoritative source; the hub's S6 (export
  structured operational telemetry, keeping loss-tolerant metrics separate from mandatory action
  facts) and S2 (linked action-record shapes) are the Server changes Sentinel depends on.
- **Warden** enforces suspensions; **Gate**, **Gateway**, **Council** and **Registry** produce the
  records the timeline reads; **Console** displays Sentinel's views and submits suspensions through
  the same Warden API; **Assure** consumes the same records for evidence packages, and both report
  gaps as gaps.
- **External dependencies.** An OpenTelemetry exporter at a pinned convention version; nothing
  else chosen.

## Not in scope

- Writing to any authority-plane state, or restoring authority.
- Being required for local enforcement to work.
- Explaining a model's reasoning.
- Filling gaps, or presenting a view without the identifiers that say what it covers.
- Equivalence to a hardware-isolated external verifier.

## Roadmap position

| Stage | Sentinel's part |
|---|---|
| 0 · month 1 | This repository; the telemetry schema and breaker contract drafted in the hub |
| 1–2 · months 2–6 | Consumes the action-record shapes as they land; no separate release |
| 3 · months 7–9 | Timeline, counters, one alert, authenticated suspension, measured propagation |
| 4 · months 10–12 | Rebuildable views and gap reporting in the reference composition; evidence for Assure |
| 5 · months 13+ | Drift comparison, SIEM and SOAR integrations, anomaly models, demand-led |

## Repository layout

| Path | What exists |
|---|---|
| [Cargo.toml](Cargo.toml), [Cargo.lock](Cargo.lock) | Independent library, version 0.1.0-dev, publishing disabled, no external crate dependencies |
| [src/lib.rs](src/lib.rs) | Documented proposed module interfaces; no runtime implementations |
| [docs/](docs/README.md) | Architecture, implementation sequence and acceptance specifications |
| [CONTRIBUTING.md](CONTRIBUTING.md), [AGENTS.md](AGENTS.md), [CLAUDE.md](CLAUDE.md) | Contribution process and aligned development guidance |
| [.github/workflows/](.github/workflows/) | Automatic Rust, repository-hygiene and DCO checks |
| [scripts/](scripts/), [check_license.py](check_license.py) | Existing documentation, private-material and license checks |
| [LICENSE](LICENSE), [NOTICE](NOTICE), [THIRD_PARTY_NOTICES.md](THIRD_PARTY_NOTICES.md) | Licensing and dependency notices |

Subsystem modules: [timeline](src/timeline.rs), [telemetry](src/telemetry.rs), [suspension](src/suspension.rs).
Tests, fixtures, migrations, binaries and deployment assets arrive with the implementation that
uses them. The scaffold defines no shared wire types and depends on no sibling checkout.

## Development

Use Rust 1.98.1 with rustfmt, Clippy and the platform's native linker. From this repository root:

```console
cargo fmt --all --check
cargo build --offline --locked
cargo clippy --offline --locked --all-targets -- -D warnings
cargo test --offline --locked
cargo doc --offline --locked --no-deps
```

The crate currently has **zero runtime or conformance tests**. A successful test command checks
the scaffold only. The [validation guide](docs/validation.md) gives the required behavioral
test specifications and explains how to retain evidence when they are implemented.

Also run the existing hygiene gates:

```console
py check_license.py
py scripts/private_material_scan.py
py scripts/docs_linkcheck.py
gitleaks dir . --config .gitleaks.toml --no-banner --redact --exit-code 1
git diff --check
```

Use `python` or `python3` where `py` is unavailable. The new
[Rust workflow](.github/workflows/rust.yml) runs on main pushes and pull requests alongside
the existing [repository hygiene](.github/workflows/repo-hygiene.yml) and
[DCO](.github/workflows/dco.yml) workflows. They provide build and repository checks, not a
qualified runtime. No package is published or service deployed by these workflows.
Local checks do not imply hosted CI success. See [CONTRIBUTING.md](CONTRIBUTING.md).

## The platform

| Repository | Plane | Role |
|---|---|---|
| [iokaio/munarium-platform](https://github.com/iokaio/munarium-platform) | hub | Architecture, normative contracts, decision records, roadmap and composition evidence for the whole platform |
| [iokaio/munarium](https://github.com/iokaio/munarium) | foundation (mediation) | Munarium Server: governed memory, the append-only ledger, and the Server client libraries |
| [iokaio/munarium-matrix](https://github.com/iokaio/munarium-matrix) | foundation (mediation) | Munarium Matrix: governed, read-only structured evidence from enterprise data sources |
| [iokaio/munarium-registry](https://github.com/iokaio/munarium-registry) | authority | Inventory of agents, tools, manifests, and policy bundles |
| [iokaio/munarium-harness](https://github.com/iokaio/munarium-harness) | agent | SDKs that make the governed path easy for honest agents |
| [iokaio/munarium-warden](https://github.com/iokaio/munarium-warden) | authority | Workload identity, delegation, just-in-time credentials, kill switches |
| [iokaio/munarium-gate](https://github.com/iokaio/munarium-gate) | mediation | Policy decision and enforcement point for every tool call |
| [iokaio/munarium-gateway](https://github.com/iokaio/munarium-gateway) | mediation | Model-call mediation: routing, BYOK, budgets, screening |
| [iokaio/munarium-council](https://github.com/iokaio/munarium-council) | authority | Approvals, policy lifecycle, ratified governance transitions |
| [iokaio/munarium-sentinel](https://github.com/iokaio/munarium-sentinel) | assurance | Telemetry, anomaly detection, circuit breakers, incident replay |
| [iokaio/munarium-assure](https://github.com/iokaio/munarium-assure) | assurance | Control-framework mapping and evidence packs |
| [iokaio/munarium-console](https://github.com/iokaio/munarium-console) | assurance | One interface for approvers, operators, and auditors |
| [iokaio/munarium-clients-publish](https://github.com/iokaio/munarium-clients-publish) | tooling | The one place Munarium client packages are built for release and published from |
| [iokaio/munarium-demo](https://github.com/iokaio/munarium-demo) | examples | Munarium Demo: working applications and bundled datasets for evaluating the foundation |

The development tool VCP ([iokaio/vcp](https://github.com/iokaio/vcp)) is separate: not one of the
nine components and not a runtime dependency for adopters. Ioka's private repositories hold
planning material awaiting publication review and the proprietary Matrix analytics adapters;
nothing from them is copied into a public repository without that review.

## Licensing

Apache-2.0 ([LICENSE](LICENSE), [NOTICE](NOTICE)). The names are not part of that grant:
[TRADEMARK.md](TRADEMARK.md) says what you may do without asking, which is most things. There is
no proprietary edition of this component and none is planned; a capability that arrives later is
deferred roadmap work, not a commercial restriction.

## Contributing, support, security

Signed-off pull requests, no CLA ([CONTRIBUTING.md](CONTRIBUTING.md)). Questions go to Discussions,
defects and design findings to Issues, and suspected vulnerabilities to the private channel
[SECURITY.md](SECURITY.md) names, never a public issue. What is and is not supported:
[SUPPORT.md](SUPPORT.md). Conduct: [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md). Release history,
such as it is: [CHANGELOG.md](CHANGELOG.md).
