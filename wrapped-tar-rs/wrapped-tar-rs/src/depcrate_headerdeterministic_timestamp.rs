// Generated macro for DETERMINISTIC_TIMESTAMP (const)
macro_rules! Depcrate_headerDETERMINISTIC_TIMESTAMP {
() => {
// Module: crate::header
// Provides: {"DETERMINISTIC_TIMESTAMP"}
// Dependencies: {}
# [doc = " A deterministic, arbitrary, non-zero timestamp that use used as `mtime`"] # [doc = " of headers when [`HeaderMode::Deterministic`] is used."] # [doc = ""] # [doc = " This value, chosen after careful deliberation, corresponds to _Jul 23, 2006_,"] # [doc = " which is the date of the first commit for what would become Rust."] # [cfg (all (any (unix , windows) , not (target_arch = "wasm32")))] const DETERMINISTIC_TIMESTAMP : u64 = 1153704088 ;
};
}
