macro_rules! deps {
    () => {
        ZipResult!();
        ZipFileData!();
    };
}

macro_rules! update_aes_extra_data {
    () => {
        deps!();
        fn update_aes_extra_data < W : Write + Seek > (writer : & mut W , file : & mut ZipFileData) -> ZipResult < () > { let Some ((aes_mode , version , compression_method)) = file . aes_mode else { return Ok (()) ; } ; let extra_data_start = file . extra_data_start . unwrap () ; writer . seek (SeekFrom :: Start (extra_data_start + file . aes_extra_data_start ,)) ? ; let mut buf = Vec :: new () ; buf . write_u16_le (0x9901) ? ; buf . write_u16_le (7) ? ; buf . write_u16_le (version as u16) ? ; buf . write_all (b"AE") ? ; buf . write_all (& [aes_mode as u8]) ? ; buf . write_u16_le (compression_method . serialize_to_u16 ()) ? ; writer . write_all (& buf) ? ; let aes_extra_data_start = file . aes_extra_data_start as usize ; let extra_field = Arc :: get_mut (file . extra_field . as_mut () . unwrap ()) . unwrap () ; extra_field [aes_extra_data_start .. aes_extra_data_start + buf . len ()] . copy_from_slice (& buf) ; Ok (()) }
    };
}

update_aes_extra_data!();