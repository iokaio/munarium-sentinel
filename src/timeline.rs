// SPDX-License-Identifier: Apache-2.0
//! Project authoritative events with coverage watermarks.
//!
//! Retain source identifiers and visible missing intervals. A projection is rebuildable and cannot become a competing accountability record.
//!
//! Proposed local interface only. No implementation or wire format is provided.

/// Proposed boundary for: project authoritative events with coverage watermarks.
///
/// Implementations and concrete types await the component design and hub contracts.
/// This declaration does not enforce authentication, authorization, or durability.
pub trait TimelineProjection {
    /// Input whose concrete shape and validation rules are still to be specified.
    type EventBatch;
    /// Output whose concrete shape and evidence requirements are still to be specified.
    type Watermark;
    /// Failure reported without manufacturing a successful or authorized result.
    type Error;

    /// Project authoritative events with coverage watermarks.
    ///
    /// # Errors
    ///
    /// Implementations must report failed validation or unavailable required dependencies.
    fn ingest(&mut self, input: &Self::EventBatch) -> Result<Self::Watermark, Self::Error>;
}
