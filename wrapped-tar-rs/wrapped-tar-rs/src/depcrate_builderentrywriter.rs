// Generated macro for EntryWriter (struct)
macro_rules! Depcrate_builderEntryWriter {
() => {
// Module: crate::builder
// Provides: {"EntryWriter"}
// Dependencies: {}
# [doc = " A writer for a single entry in a tar archive."] # [doc = ""] # [doc = " This struct is returned by [`Builder::append_writer`] and provides a"] # [doc = " [`Write`] implementation for adding content to an archive entry."] # [doc = ""] # [doc = " After writing all data to the entry, it must be finalized either by"] # [doc = " explicitly calling [`EntryWriter::finish`] or by letting it drop."] pub struct EntryWriter < 'a > { obj : & 'a mut dyn SeekWrite , header : & 'a mut Header , written : u64 , }
};
}
