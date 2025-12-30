// Generated macro for Entry (struct)
macro_rules! Depcrate_entryEntry {
() => {
// Module: crate::entry
// Provides: {"Entry"}
// Dependencies: {}
# [doc = " A read-only view into an entry of an archive."] # [doc = ""] # [doc = " This structure is a window into a portion of a borrowed archive which can"] # [doc = " be inspected. It acts as a file handle by implementing the Reader trait. An"] # [doc = " entry cannot be rewritten once inserted into an archive."] pub struct Entry < 'a , R : 'a + Read > { fields : EntryFields < 'a > , _ignored : marker :: PhantomData < & 'a Archive < R > > , }
};
}
