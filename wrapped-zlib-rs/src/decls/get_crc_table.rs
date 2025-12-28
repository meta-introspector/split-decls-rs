macro_rules! get_crc_table {
    () => {
        pub fn get_crc_table () -> & 'static [u32 ; 256] { braid :: get_crc_table () }
    };
}

get_crc_table!()