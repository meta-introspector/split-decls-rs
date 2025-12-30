// Generated macro for EntryIo (enum)
macro_rules! Depcrate_entryEntryIo {
() => {
// Module: crate::entry
// Provides: {"EntryIo"}
// Dependencies: {}
pub enum EntryIo < 'a > { Pad (io :: Take < io :: Repeat >) , Data (io :: Take < & 'a ArchiveInner < dyn Read + 'a > >) , }
};
}
