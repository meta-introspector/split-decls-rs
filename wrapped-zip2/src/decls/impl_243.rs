macro_rules! deps {
    () => {
        ZipFileData!();
        LittleEndianReadExt!();
        ExtendedFileOptions!();
        ZipResult!();
        ZipError!();
    };
}

macro_rules! impl_243 {
    () => {
        deps!();
        impl ExtendedFileOptions { # [doc = " Adds an extra data field, unless we detect that it's invalid."] pub fn add_extra_data < D : AsRef < [u8] > > (& mut self , header_id : u16 , data : D , central_only : bool ,) -> ZipResult < () > { let data = data . as_ref () ; let len = data . len () + 4 ; if self . extra_data . len () + self . central_extra_data . len () + len > u16 :: MAX as usize { Err (invalid ! ("Extra data field would be longer than allowed")) } else { let field = if central_only { & mut self . central_extra_data } else { & mut self . extra_data } ; let vec = Arc :: get_mut (field) ; let vec = match vec { Some (exclusive) => exclusive , None => { * field = Arc :: new (field . to_vec ()) ; Arc :: get_mut (field) . unwrap () } } ; Self :: add_extra_data_unchecked (vec , header_id , data) ? ; Self :: validate_extra_data (vec , true) ? ; Ok (()) } } pub (crate) fn add_extra_data_unchecked (vec : & mut Vec < u8 > , header_id : u16 , data : & [u8] ,) -> Result < () , ZipError > { vec . reserve_exact (data . len () + 4) ; vec . write_u16_le (header_id) ? ; vec . write_u16_le (data . len () as u16) ? ; vec . write_all (data) ? ; Ok (()) } fn validate_extra_data (data : & [u8] , disallow_zip64 : bool) -> ZipResult < () > { let len = data . len () as u64 ; if len == 0 { return Ok (()) ; } if len > u16 :: MAX as u64 { return Err (ZipError :: Io (io :: Error :: other ("Extra-data field can't exceed u16::MAX bytes" ,))) ; } let mut data = Cursor :: new (data) ; let mut pos = data . position () ; while pos < len { if len - data . position () < 4 { return Err (ZipError :: Io (io :: Error :: other ("Extra-data field doesn't have room for ID and length" ,))) ; } # [cfg (not (feature = "unreserved"))] { use crate :: unstable :: LittleEndianReadExt ; let header_id = data . read_u16_le () ? ; if EXTRA_FIELD_MAPPING . contains (& header_id) { return Err (ZipError :: Io (io :: Error :: other (format ! ("Extra data header ID {header_id:#06} requires crate feature \"unreserved\"" ,) ,))) ; } data . seek (SeekFrom :: Current (- 2)) ? ; } parse_single_extra_field (& mut ZipFileData :: default () , & mut data , pos , disallow_zip64) ? ; pos = data . position () ; } Ok (()) } }
    };
}

impl_243!()