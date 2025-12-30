// Generated macro for Unpacked (enum)
macro_rules! Depcrate_entryUnpacked {
() => {
// Module: crate::entry
// Provides: {"Unpacked"}
// Dependencies: {}
# [doc = " When unpacking items the unpacked thing is returned to allow custom"] # [doc = " additional handling by users. Today the File is returned, in future"] # [doc = " the enum may be extended with kinds for links, directories etc."] # [derive (Debug)] pub enum Unpacked { # [doc = " A file was unpacked."] File (std :: fs :: File) , # [doc = " A directory, hardlink, symlink, or other node was unpacked."] # [doc (hidden)] __Nonexhaustive , }
};
}
