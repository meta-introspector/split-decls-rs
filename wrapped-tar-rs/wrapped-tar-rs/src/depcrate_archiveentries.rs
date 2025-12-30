// Generated macro for Entries (struct)
macro_rules! Depcrate_archiveEntries {
() => {
// Module: crate::archive
// Provides: {"Entries"}
// Dependencies: {}
# [doc = " An iterator over the entries of an archive."] pub struct Entries < 'a , R : 'a + Read > { fields : EntriesFields < 'a > , _ignored : marker :: PhantomData < & 'a Archive < R > > , }
};
}
