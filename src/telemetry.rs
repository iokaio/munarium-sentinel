// SPDX-License-Identifier: Apache-2.0
//! Export redacted operational observations.
//!
//! Pin the telemetry convention in the implementation; loss-tolerant metrics do not replace mandatory action records.
//!
//! Proposed local interface only. No implementation or wire format is provided.

/// Proposed boundary for: export redacted operational observations.
///
/// Implementations and concrete types await the component design and hub contracts.
/// This declaration does not enforce authentication, authorization, or durability.
pub trait TelemetryExporter {
    /// Input whose concrete shape and validation rules are still to be specified.
    type ObservationBatch;
    /// Output whose concrete shape and evidence requirements are still to be specified.
    type ExportReceipt;
    /// Failure reported without manufacturing a successful or authorized result.
    type Error;

    /// Export redacted operational observations.
    ///
    /// # Errors
    ///
    /// Implementations must report failed validation or unavailable required dependencies.
    fn export(
        &mut self,
        input: &Self::ObservationBatch,
    ) -> Result<Self::ExportReceipt, Self::Error>;
}
