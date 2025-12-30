// Generated macro for Ancestor (struct)
macro_rules! DepcrateAncestor {
() => {
// Module: crate
// Provides: {"Ancestor"}
// Dependencies: {}
# [doc = " An ancestor is an item in the directory tree traversed by walkdir, and is"] # [doc = " used to check for loops in the tree when traversing symlinks."] # [derive (Debug)] struct Ancestor { # [doc = " The path of this ancestor."] path : PathBuf , # [doc = " An open file to this ancesor. This is only used on Windows where"] # [doc = " opening a file handle appears to be quite expensive, so we choose to"] # [doc = " cache it. This comes at the cost of not respecting the file descriptor"] # [doc = " limit set by the user."] # [cfg (windows)] handle : Handle , }
};
}
