// Generated macro for UtcOffset (struct)
macro_rules! Depcrate_utc_offsetUtcOffset {
() => {
// Module: crate::utc_offset
// Provides: {"UtcOffset"}
// Dependencies: {}
# [doc = " An offset from UTC."] # [doc = ""] # [doc = " This struct can store values up to ±25:59:59. If you need support outside this range, please"] # [doc = " file an issue with your use case."] # [derive (Clone , Copy , Eq)] # [cfg_attr (not (docsrs) , repr (C))] pub struct UtcOffset { # [doc = " The order of this struct's fields matter. Do not reorder them."] # [cfg (target_endian = "little")] seconds : Seconds , # [cfg (target_endian = "little")] minutes : Minutes , # [cfg (target_endian = "little")] hours : Hours , # [cfg (target_endian = "big")] hours : Hours , # [cfg (target_endian = "big")] minutes : Minutes , # [cfg (target_endian = "big")] seconds : Seconds , }
};
}
