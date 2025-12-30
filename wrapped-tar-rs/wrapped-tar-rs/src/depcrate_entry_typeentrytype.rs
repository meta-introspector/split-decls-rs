// Generated macro for EntryType (enum)
macro_rules! Depcrate_entry_typeEntryType {
() => {
// Module: crate::entry_type
// Provides: {"EntryType"}
// Dependencies: {}
# [doc = " Indicate the type of content described by a header."] # [doc = ""] # [doc = " This is returned by [`crate::Header::entry_type()`] and should be used to"] # [doc = " distinguish between types of content."] # [derive (Clone , Copy , PartialEq , Eq , Debug)] pub enum EntryType { # [doc = " Regular file"] Regular , # [doc = " Hard link"] Link , # [doc = " Symbolic link"] Symlink , # [doc = " Character device"] Char , # [doc = " Block device"] Block , # [doc = " Directory"] Directory , # [doc = " Named pipe (fifo)"] Fifo , # [doc = " Implementation-defined 'high-performance' type, treated as regular file"] Continuous , # [doc = " GNU extension - long file name"] GNULongName , # [doc = " GNU extension - long link name (link target)"] GNULongLink , # [doc = " GNU extension - sparse file"] GNUSparse , # [doc = " Global extended header"] XGlobalHeader , # [doc = " Extended Header"] XHeader , # [doc = " Hints that destructuring should not be exhaustive."] # [doc = ""] # [doc = " This enum may grow additional variants, so this makes sure clients"] # [doc = " don't count on exhaustive matching. (Otherwise, adding a new variant"] # [doc = " could break existing code.)"] # [doc (hidden)] __Nonexhaustive (u8) , }
};
}
