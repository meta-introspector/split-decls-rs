// Generated macro for EntriesFields (struct)
macro_rules! Depcrate_archiveEntriesFields {
() => {
// Module: crate::archive
// Provides: {"EntriesFields"}
// Dependencies: {}
struct EntriesFields < 'a > { archive : & 'a Archive < dyn Read + 'a > , seekable_archive : Option < & 'a Archive < dyn SeekRead + 'a > > , next : u64 , done : bool , raw : bool , }
};
}
