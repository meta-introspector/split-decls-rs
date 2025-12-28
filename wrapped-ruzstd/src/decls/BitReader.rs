macro_rules! BitReader {
    () => {
        # [doc = " Wraps a slice and enables reading arbitrary amounts of bits"] # [doc = " from that slice."] pub struct BitReader < 's > { idx : usize , source : & 's [u8] , }
    };
}

BitReader!();