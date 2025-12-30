// Generated macro for DirList (enum)
macro_rules! DepcrateDirList {
() => {
// Module: crate
// Provides: {"DirList"}
// Dependencies: {}
# [doc = " A sequence of unconsumed directory entries."] # [doc = ""] # [doc = " This represents the opened or closed state of a directory handle. When"] # [doc = " open, future entries are read by iterating over the raw `fs::ReadDir`."] # [doc = " When closed, all future entries are read into memory. Iteration then"] # [doc = " proceeds over a [`Vec<fs::DirEntry>`]."] # [doc = ""] # [doc = " [`fs::ReadDir`]: https://doc.rust-lang.org/stable/std/fs/struct.ReadDir.html"] # [doc = " [`Vec<fs::DirEntry>`]: https://doc.rust-lang.org/stable/std/vec/struct.Vec.html"] # [derive (Debug)] enum DirList { # [doc = " An opened handle."] # [doc = ""] # [doc = " This includes the depth of the handle itself."] # [doc = ""] # [doc = " If there was an error with the initial [`fs::read_dir`] call, then it"] # [doc = " is stored here. (We use an [`Option<...>`] to make yielding the error"] # [doc = " exactly once simpler.)"] # [doc = ""] # [doc = " [`fs::read_dir`]: https://doc.rust-lang.org/stable/std/fs/fn.read_dir.html"] # [doc = " [`Option<...>`]: https://doc.rust-lang.org/stable/std/option/enum.Option.html"] Opened { depth : usize , it : result :: Result < ReadDir , Option < Error > > } , # [doc = " A closed handle."] # [doc = ""] # [doc = " All remaining directory entries are read into memory."] Closed (vec :: IntoIter < Result < DirEntry > >) , }
};
}
