// Generated macro for read_encoded_offset (function)
macro_rules! Depcrate_sys_personality_dwarf_ehread_encoded_offset {
() => {
// Module: crate::sys::personality::dwarf::eh
// Provides: {"read_encoded_offset"}
// Dependencies: {}
# [doc = " Reads an offset (`usize`) from `reader` whose encoding is described by `encoding`."] # [doc = ""] # [doc = " `encoding` must be a [DWARF Exception Header Encoding as described by the LSB spec][LSB-dwarf-ext]."] # [doc = " In addition the upper (\"application\") part must be zero."] # [doc = ""] # [doc = " # Errors"] # [doc = " Returns `Err` if `encoding`"] # [doc = " * is not a valid DWARF Exception Header Encoding,"] # [doc = " * is `DW_EH_PE_omit`, or"] # [doc = " * has a non-zero application part."] # [doc = ""] # [doc = " [LSB-dwarf-ext]: https://refspecs.linuxfoundation.org/LSB_5.0.0/LSB-Core-generic/LSB-Core-generic/dwarfext.html"] unsafe fn read_encoded_offset (reader : & mut DwarfReader , encoding : u8) -> Result < usize , () > { if encoding == DW_EH_PE_omit || encoding & 0xF0 != 0 { return Err (()) ; } let result = unsafe { match encoding & 0x0F { DW_EH_PE_absptr => reader . read :: < usize > () , DW_EH_PE_uleb128 => reader . read_uleb128 () as usize , DW_EH_PE_udata2 => reader . read :: < u16 > () as usize , DW_EH_PE_udata4 => reader . read :: < u32 > () as usize , DW_EH_PE_udata8 => reader . read :: < u64 > () as usize , DW_EH_PE_sleb128 => reader . read_sleb128 () as usize , DW_EH_PE_sdata2 => reader . read :: < i16 > () as usize , DW_EH_PE_sdata4 => reader . read :: < i32 > () as usize , DW_EH_PE_sdata8 => reader . read :: < i64 > () as usize , _ => return Err (()) , } } ; Ok (result) }
};
}
