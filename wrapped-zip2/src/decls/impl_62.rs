macro_rules! deps {
    () => {
        ExtendedTimestamp!();
        ZipResult!();
        ZipError!();
    };
}

macro_rules! impl_62 {
    () => {
        deps!();
        impl ExtendedTimestamp { # [doc = " creates an extended timestamp struct by reading the required bytes from the reader."] # [doc = ""] # [doc = " This method assumes that the length has already been read, therefore"] # [doc = " it must be passed as an argument"] pub fn try_from_reader < R > (reader : & mut R , len : u16) -> ZipResult < Self > where R : Read , { if len == 0 { return Err (invalid ! ("Extended timestamp field is empty")) ; } let mut flags = [0u8] ; let mut bytes_to_read = len as usize ; reader . read_exact (& mut flags) ? ; bytes_to_read -= flags . len () ; let flags = flags [0] ; if len != 5 && len as u32 != 1 + 4 * flags . count_ones () { return Err (ZipError :: UnsupportedArchive ("flags and len don't match in extended timestamp field" ,)) ; } let mod_time = if (flags & 0b00000001u8 == 0b00000001u8) || len == 5 { bytes_to_read -= size_of :: < u32 > () ; Some (reader . read_u32_le () ?) } else { None } ; let ac_time = if flags & 0b00000010u8 == 0b00000010u8 && len > 5 { bytes_to_read -= size_of :: < u32 > () ; Some (reader . read_u32_le () ?) } else { None } ; let cr_time = if flags & 0b00000100u8 == 0b00000100u8 && len > 5 { bytes_to_read -= size_of :: < u32 > () ; Some (reader . read_u32_le () ?) } else { None } ; if bytes_to_read > 0 { reader . read_exact (& mut vec ! [0 ; bytes_to_read]) ? } Ok (Self { mod_time , ac_time , cr_time , }) } # [doc = " returns the last modification timestamp, if defined, as UNIX epoch seconds"] pub fn mod_time (& self) -> Option < u32 > { self . mod_time } # [doc = " returns the last access timestamp, if defined, as UNIX epoch seconds"] pub fn ac_time (& self) -> Option < u32 > { self . ac_time } # [doc = " returns the creation timestamp, if defined, as UNIX epoch seconds"] pub fn cr_time (& self) -> Option < u32 > { self . cr_time } }
    };
}

impl_62!();