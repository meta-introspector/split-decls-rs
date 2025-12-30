// Generated macro for WalkDirOptions (struct)
macro_rules! DepcrateWalkDirOptions {
() => {
// Module: crate
// Provides: {"WalkDirOptions"}
// Dependencies: {}
struct WalkDirOptions { follow_links : bool , follow_root_links : bool , max_open : usize , min_depth : usize , max_depth : usize , sorter : Option < Box < dyn FnMut (& DirEntry , & DirEntry) -> Ordering + Send + Sync + 'static , > , > , contents_first : bool , same_file_system : bool , }
};
}
