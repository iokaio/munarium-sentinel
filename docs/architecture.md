# Munarium Sentinel implementation architecture

**Proposed design; scaffold only.** Based on section 13 of the
[platform plan, revision 4](https://github.com/iokaio/munarium-platform/blob/main/docs/platform-plan.md), with lifecycle and failure rules in
sections 17–19 and 22. See the hub's
[scaffold decision proposal](https://github.com/iokaio/munarium-platform/blob/main/docs/decisions/0001-scaffold-boundaries.md)
for the distinction between local interfaces and normative contracts.

## Responsibility and current boundary

Rebuildable event timelines, explicit evidence gaps, and bounded suspension requests. Sentinel belongs to the **assurance plane**.
The crate declares interfaces only: no concrete implementations, serialization,
network listeners, persistence, service authentication or target operations exist.

The associated input, output and error types are intentionally unspecified.
These are proposed in-process seams for implementation work, not a released Rust API
or a second definition of the shared wire contract. A trait signature does not enforce
the trust assumptions below. Async runtime, transport and storage choices remain open.

## Module map

| Source | Proposed interface | Responsibility |
|---|---|---|
| [timeline](../src/timeline.rs) | `TimelineProjection` | Retain source identifiers and visible missing intervals. A projection is rebuildable and cannot become a competing accountability record. |
| [telemetry](../src/telemetry.rs) | `TelemetryExporter` | Pin the telemetry convention in the implementation; loss-tolerant metrics do not replace mandatory action records. |
| [suspension](../src/suspension.rs) | `SuspensionClient` | A submitted request is not proof of enforced suspension. No automatic restoration or policy-editing method belongs on this port. |

## Planned flow and state ownership

Read authoritative action events → project timelines with source IDs/watermarks → expose gaps and basic counters → identify a bounded trigger → submit authenticated suspension to Warden → observe grant rejection evidence. Local hard controls operate independently of the dashboard.

Own disposable projections, cursors and export progress. Preserve references to source records. Warden owns enforced suspension state; an alert, sent request, or UI label cannot replace its acknowledgement and measured propagation.

## Dependencies and failure behavior

| Dependency | Required input or service | Failure rule |
|---|---|---|
| Server S2 / S6 | Versioned authoritative events and durable source positions | Show a gap or stale watermark; do not fabricate an empty successful interval. |
| Gate / Gateway | Action and budget observations | Loss-tolerant metrics do not replace required facts. |
| Warden | Authenticated bounded suspension API and enforcement evidence | Unacknowledged request is pending/failed, not enforced. |
| Telemetry consumer | Explicit pinned convention and payload policy | Export failures remain visible without weakening local authorization. |

No dependency is linked into this scaffold. Supported contract versions are **none**.
Future adapters must consume a reviewed, versioned contract and identify its digest;
a floating hub branch is design context, never deployment authority.

## Threat assumptions

Treat agent code, supplied content and self-reported identity as untrusted.
Host administrators, release roots and required signing authorities remain explicit
trust assumptions of a qualified deployment. Process separation alone does not prove
independent administration.

| Threat | Required control to implement and test |
|---|---|
| Missing telemetry mistaken for health | Coverage and watermarks stay explicit in every projection. |
| Overprivileged automation | Only pre-authorized narrowing through Warden; no authority restoration. |
| Index tampering or payload disclosure | Rebuild from authoritative source and export redacted references. |

The [validation specification](validation.md) connects these requirements to the hub
invariants. No test evidence is implied by this design.

## Decisions needed before implementation

Define event ordering, watermark/rebuild semantics, redaction policy and a pinned telemetry convention. Agree suspension scope, duration, acknowledgement and measured revocation bounds with Warden.

A cross-component semantic change starts in a hub decision record. Keep publication,
activation and component implementation separate. Use expand, migrate, remove for
future breaking contract changes; never duplicate hashing, identity or grant rules.

## Deferred scope

Sophisticated anomaly models, many SIEM/SOAR adapters, and any hardware-isolated assurance claim.

The [implementation plan](implementation-plan.md) sequences the first useful increment.
No deployment recipe, service port or live-provider configuration is supplied at this stage.
