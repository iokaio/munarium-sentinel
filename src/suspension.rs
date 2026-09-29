// SPDX-License-Identifier: Apache-2.0
//! Request a pre-authorized narrowing of capability through Warden.
//!
//! A submitted request is not proof of enforced suspension. No automatic restoration or policy-editing method belongs on this port.
//!
//! Proposed local interface only. No implementation or wire format is provided.

/// Proposed boundary for: request a pre-authorized narrowing of capability through Warden.
///
/// Implementations and concrete types await the component design and hub contracts.
/// This declaration does not enforce authentication, authorization, or durability.
pub trait SuspensionClient {
    /// Input whose concrete shape and validation rules are still to be specified.
    type SuspensionRequest;
    /// Output whose concrete shape and evidence requirements are still to be specified.
    type SuspensionReceipt;
    /// Failure reported without manufacturing a successful or authorized result.
    type Error;

    /// Request a pre-authorized narrowing of capability through Warden.
    ///
    /// # Errors
    ///
    /// Implementations must report failed validation or unavailable required dependencies.
    fn request(
        &mut self,
        input: &Self::SuspensionRequest,
    ) -> Result<Self::SuspensionReceipt, Self::Error>;
}
