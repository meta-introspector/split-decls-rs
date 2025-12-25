use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Answers "why wasn't the source type transmutable into the destination type?"
#[derive(Debug, Hash, Eq, PartialEq, PartialOrd, Ord, Clone)]
pub enum Reason<T> {
    /// The layout of the source type is not yet supported.
    SrcIsNotYetSupported,
    /// The layout of the destination type is not yet supported.
    DstIsNotYetSupported,
    /// The layout of the destination type is bit-incompatible with the source type.
    DstIsBitIncompatible,
    /// The destination type is uninhabited.
    DstUninhabited,
    /// The destination type may carry safety invariants.
    DstMayHaveSafetyInvariants,
    /// `Dst` is larger than `Src`, and the excess bytes were not exclusively uninitialized.
    DstIsTooBig,
    /// `Dst` is larger `Src`.
    DstRefIsTooBig {
        /// The referent of the source type.
        src: T,
        /// The size of the source type's referent.
        src_size: usize,
        /// The too-large referent of the destination type.
        dst: T,
        /// The size of the destination type's referent.
        dst_size: usize,
    },
    /// Src should have a stricter alignment than Dst, but it does not.
    DstHasStricterAlignment {
        src_min_align: usize,
        dst_min_align: usize,
    },
    /// Can't go from shared pointer to unique pointer
    DstIsMoreUnique,
    /// Encountered a type error
    TypeError,
    /// The layout of src is unknown
    SrcLayoutUnknown,
    /// The layout of dst is unknown
    DstLayoutUnknown,
    /// The size of src is overflow
    SrcSizeOverflow,
    /// The size of dst is overflow
    DstSizeOverflow,
}
