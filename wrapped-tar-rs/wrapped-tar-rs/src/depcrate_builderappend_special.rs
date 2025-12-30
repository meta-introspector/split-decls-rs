// Generated macro for append_special (function)
macro_rules! Depcrate_builderappend_special {
() => {
// Module: crate::builder
// Provides: {"append_special"}
// Dependencies: {}
# [cfg (unix)] fn append_special (dst : & mut dyn Write , path : & Path , stat : & fs :: Metadata , mode : HeaderMode ,) -> io :: Result < () > { use :: std :: os :: unix :: fs :: { FileTypeExt , MetadataExt } ; let file_type = stat . file_type () ; let entry_type ; if file_type . is_socket () { return Err (other (& format ! ("{}: socket can not be archived" , path . display ()))) ; } else if file_type . is_fifo () { entry_type = EntryType :: Fifo ; } else if file_type . is_char_device () { entry_type = EntryType :: Char ; } else if file_type . is_block_device () { entry_type = EntryType :: Block ; } else { return Err (other (& format ! ("{} has unknown file type" , path . display ()))) ; } let mut header = Header :: new_gnu () ; header . set_metadata_in_mode (stat , mode) ; prepare_header_path (dst , & mut header , path) ? ; header . set_entry_type (entry_type) ; let dev_id = stat . rdev () ; let dev_major = ((dev_id >> 32) & 0xffff_f000) | ((dev_id >> 8) & 0x0000_0fff) ; let dev_minor = ((dev_id >> 12) & 0xffff_ff00) | ((dev_id) & 0x0000_00ff) ; header . set_device_major (dev_major as u32) ? ; header . set_device_minor (dev_minor as u32) ? ; header . set_cksum () ; dst . write_all (header . as_bytes ()) ? ; Ok (()) }
};
}
