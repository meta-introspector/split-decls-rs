// Generated macro for OwnedInternal (enum)
macro_rules! Depcrate_internal_ownedOwnedInternal {
() => {
// Module: crate::internal::owned
// Provides: {"OwnedInternal"}
// Dependencies: {}
# [derive (Clone)] pub (crate) enum OwnedInternal { BigSigned (i128) , BigUnsigned (u128) , Float (f64) , Bool (bool) , Char (char) , # [cfg (feature = "inline-str")] StrSmall (inline_str :: InlineStr) , Str (Box < str >) , None , Debug (internal :: fmt :: owned :: OwnedFmt) , Display (internal :: fmt :: owned :: OwnedFmt) , # [cfg (feature = "error")] Error (internal :: error :: owned :: OwnedError) , # [cfg (feature = "serde1")] Serde1 (internal :: serde :: v1 :: owned :: OwnedSerialize) , # [cfg (feature = "sval2")] Sval2 (internal :: sval :: v2 :: owned :: OwnedValue) , # [cfg (feature = "seq")] Seq (internal :: seq :: owned :: OwnedSeq) , SharedDebug (Arc < dyn internal :: fmt :: DowncastDebug + Send + Sync >) , SharedDisplay (Arc < dyn internal :: fmt :: DowncastDisplay + Send + Sync >) , # [cfg (feature = "error")] SharedError (Arc < dyn internal :: error :: DowncastError + Send + Sync >) , # [cfg (feature = "serde1")] SharedSerde1 (Arc < dyn internal :: serde :: v1 :: DowncastSerialize + Send + Sync >) , # [cfg (feature = "sval2")] SharedSval2 (Arc < dyn internal :: sval :: v2 :: DowncastValue + Send + Sync >) , # [cfg (feature = "seq")] SharedSeq (Arc < dyn internal :: seq :: DowncastSeq + Send + Sync >) , Poisoned (& 'static str) , }
};
}
