// Generated macro for Vfs (struct)
macro_rules! DepcrateVfs {
() => {
// Module: crate
// Provides: {"Vfs"}
// Dependencies: {}
# [doc = " Storage for all file changes and the file id to path mapping."] # [doc = ""] # [doc = " For more information see the [crate-level](crate) documentation."] # [derive (Default)] pub struct Vfs { interner : PathInterner , data : Vec < FileState > , changes : IndexMap < FileId , ChangedFile , BuildHasherDefault < FxHasher > > , }
};
}
