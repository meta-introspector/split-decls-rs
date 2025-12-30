// Generated macro for FlockType (enum)
macro_rules! Depcrate_process_typesFlockType {
() => {
// Module: crate::process::types
// Provides: {"FlockType"}
// Dependencies: {}
# [doc = " `F_*LCK` constants for use with [`fcntl_getlk`]."] # [doc = ""] # [doc = " [`fcntl_getlk`]: crate::process::fcntl_getlk()"] # [cfg (not (target_os = "horizon"))] # [derive (Clone , Copy , Debug , PartialEq , Eq)] # [repr (i16)] pub enum FlockType { # [doc = " `F_RDLCK`"] ReadLock = c :: F_RDLCK as _ , # [doc = " `F_WRLCK`"] WriteLock = c :: F_WRLCK as _ , # [doc = " `F_UNLCK`"] Unlocked = c :: F_UNLCK as _ , }
};
}
