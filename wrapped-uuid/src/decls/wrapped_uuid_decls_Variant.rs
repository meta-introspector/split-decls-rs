use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// The reserved variants of UUIDs.
///
/// # References
///
/// * [Variant Field in RFC 9562](https://www.ietf.org/rfc/rfc9562.html#section-4.1)
#[derive(Clone, Copy, Debug, PartialEq)]
#[non_exhaustive]
#[repr(u8)]
pub enum Variant {
    /// Reserved by the NCS for backward compatibility.
    NCS = 0u8,
    /// As described in the RFC 9562 Specification (default).
    /// (for backward compatibility it is not yet renamed)
    RFC4122,
    /// Reserved by Microsoft for backward compatibility.
    Microsoft,
    /// Reserved for future expansion.
    Future,
}
