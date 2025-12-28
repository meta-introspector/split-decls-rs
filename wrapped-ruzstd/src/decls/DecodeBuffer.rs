macro_rules! deps {
    () => {
        RingBuffer!();
    };
}

macro_rules! DecodeBuffer {
    () => {
        deps!();
        pub struct DecodeBuffer { buffer : RingBuffer , pub dict_content : Vec < u8 > , pub window_size : usize , total_output_counter : u64 , # [cfg (feature = "hash")] pub hash : twox_hash :: XxHash64 , }
    };
}

DecodeBuffer!();