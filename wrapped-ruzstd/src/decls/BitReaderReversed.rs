macro_rules! BitReaderReversed {
    () => {
        # [doc = " Zstandard encodes some types of data in a way that the data must be read"] # [doc = " back to front to decode it properly. `BitReaderReversed` provides a"] # [doc = " convenient interface to do that."] pub struct BitReaderReversed < 's > { # [doc = " The index of the last read byte in the source."] index : usize , # [doc = " How many bits have been consumed from `bit_container`."] bits_consumed : u8 , # [doc = " How many bits have been consumed past the end of the input. Will be zero until all the input"] # [doc = " has been read."] extra_bits : usize , # [doc = " The source data to read from."] source : & 's [u8] , # [doc = " The reader doesn't read directly from the source, it reads bits from here, and the container"] # [doc = " is \"refilled\" as it's emptied."] bit_container : u64 , }
    };
}

BitReaderReversed!()