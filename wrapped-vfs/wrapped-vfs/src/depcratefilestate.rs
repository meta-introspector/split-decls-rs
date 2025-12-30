// Generated macro for FileState (enum)
macro_rules! DepcrateFileState {
() => {
// Module: crate
// Provides: {"FileState"}
// Dependencies: {}
# [derive (Copy , Clone , Debug , PartialEq , PartialOrd)] pub enum FileState { # [doc = " The file exists with the given content hash."] Exists (u64) , # [doc = " The file is deleted."] Deleted , # [doc = " The file was specifically excluded by the user. We still include excluded files"] # [doc = " when they're opened (without their contents)."] Excluded , }
};
}
