// Generated macro for PathInterner (struct)
macro_rules! Depcrate_path_internerPathInterner {
() => {
// Module: crate::path_interner
// Provides: {"PathInterner"}
// Dependencies: {}
# [doc = " Structure to map between [`VfsPath`] and [`FileId`]."] # [derive (Default)] pub (crate) struct PathInterner { map : IndexSet < VfsPath , BuildHasherDefault < FxHasher > > , }
};
}
