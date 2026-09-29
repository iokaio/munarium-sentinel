# Security

Do not file a vulnerability as an issue or a pull request.

Report a suspected vulnerability in anything in this repository privately, by either route:

- GitHub's private vulnerability reporting ("Report a vulnerability" under the Security tab), or
- email to **info@ioka.io** with "security" in the subject.

Say what you found, where, and how to reproduce it. Do not include live credentials, customer data,
or a proof of concept run against a system you do not operate. You will get an acknowledgement
within two business days, and a fix, or a recorded decision, on the affected path before any related
release. Credit is given if you ask for it.

## Supported versions

Munarium Sentinel has no release. Until the first tagged release, `main` is the only line and a fix
lands there. Once releases exist, security fixes go to the current minor release and to the previous
one for six months after its successor ships; an older release gets a fix only where the
vulnerability is in a contract it still speaks.

A finding in the design is welcome now, through the same private channel if it has security
consequences and as an ordinary issue otherwise. The threat model this component is built against
is in [README.md](README.md) and, for the platform as a whole, in the hub
([iokaio/munarium-platform](https://github.com/iokaio/munarium-platform)).

## What matters most here

As runtime behavior is implemented, these are the classes of finding taken most seriously and most
quickly:

- **A view that says suspended while the action path still accepts work.** The measured interval from an authenticated suspension request to the rejection of new affected grants is Sentinel's most important contract.
- **A breaker that restores broader authority**, automatically or because an anomaly score fell. Sentinel may narrow capability under a pre-authorized policy; restoration follows the human or deterministic approval path.
- **A missing interval silently filled**, or a rebuilt view that loses the source identifiers and watermarks needed to say what it covers.
- **Sensitive payloads in telemetry export** where the deployment policy says they are excluded or redacted.
- **A telemetry connector, SIEM or SOAR integration acting as a governance administrator** rather than submitting the same bounded, authenticated suspension request as any other caller.

## What is deliberate, and is not a defect

- **Sentinel records the system's inputs and actions, not the model's internal beliefs.** A timeline shows the evidence and policy versions observed at each decision; a report that Sentinel "cannot explain what the model was thinking" describes the design.
- **Local enforcement does not depend on Sentinel being available.** Gate's authorization and Gateway's hard budget checks continue without a dashboard; where an action explicitly requires current monitoring, that requirement is an admission obligation that fails closed.
- **Munarium Sentinel is a software assurance component.** It is not a claim of equivalence to the hardware-isolated External Sentinel described in Jamey Kistner's *The Sovereign Stack*, which the platform plan acknowledges separately.

When a local development profile exists, its test identity provider, test broker, disposable target
and generated sample credentials are development conveniences confined to that profile. They are
not vulnerabilities in themselves. A path by which they reach a production deployment unnoticed is.

## Findings that cross components

A contract ambiguity that lets two components disagree about authority, a canonicalization
difference between clients, or a gap between what a release advertises and what its evidence
supports is still a security finding. Report it here, or to any other Munarium repository, through
the same private channel; it is routed to the hub and the affected repositories together. Do not
open a public issue for it in the hub.

## Secrets

If you have committed a token or key, treat it as compromised: rotate it first, then report it.
