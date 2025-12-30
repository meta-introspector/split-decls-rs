// Generated macro for FREE_CODE_QUEUE_SIZE (const)
macro_rules! Depcrate_legacy_shrinkFREE_CODE_QUEUE_SIZE {
() => {
// Module: crate::legacy::shrink
// Provides: {"FREE_CODE_QUEUE_SIZE"}
// Dependencies: {}
# [doc = " Number of codes available for the LZW dictionary (excluding control codes)"] # [doc = " These are the codes from CONTROL_CODE+1 to MAX_CODE"] const FREE_CODE_QUEUE_SIZE : usize = MAX_CODE - CONTROL_CODE + 1 ;
};
}
