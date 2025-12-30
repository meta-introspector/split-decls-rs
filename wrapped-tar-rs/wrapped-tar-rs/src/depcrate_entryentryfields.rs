// Generated macro for EntryFields (struct)
macro_rules! Depcrate_entryEntryFields {
() => {
// Module: crate::entry
// Provides: {"EntryFields"}
// Dependencies: {}
pub struct EntryFields < 'a > { pub long_pathname : Option < Vec < u8 > > , pub long_linkname : Option < Vec < u8 > > , pub pax_extensions : Option < Vec < u8 > > , pub mask : u32 , pub header : Header , pub size : u64 , pub header_pos : u64 , pub file_pos : u64 , pub data : Vec < EntryIo < 'a > > , pub unpack_xattrs : bool , pub preserve_permissions : bool , pub preserve_ownerships : bool , pub preserve_mtime : bool , pub overwrite : bool , }
};
}
