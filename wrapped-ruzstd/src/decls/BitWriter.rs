macro_rules! deps {
    () => {
        Write!();
    };
}

macro_rules! BitWriter {
    () => {
        deps!();
        # [doc = " An interface for writing an arbitrary number of bits into a buffer. Write new bits into the buffer with `write_bits`, and"] # [doc = " obtain the output using `dump`."] # [derive (Debug)] pub (crate) struct BitWriter < V : AsMut < Vec < u8 > > > { # [doc = " The buffer that's filled with bits"] output : V , # [doc = " holds a partially filled byte which gets put in outpu when it's fill with a write_bits call"] partial : u64 , bits_in_partial : usize , # [doc = " The index pointing to the next unoccupied bit. Effectively just"] # [doc = " the number of bits that have been written into the buffer so far."] bit_idx : usize , }
    };
}

BitWriter!()