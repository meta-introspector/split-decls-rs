macro_rules! deps {
    () => {
        SliceVec!();
    };
}

macro_rules! impl_106 {
    () => {
        deps!();
        # [cfg (feature = "experimental_write_impl")] impl < 's > core :: fmt :: Write for SliceVec < 's , u8 > { fn write_str (& mut self , s : & str) -> core :: fmt :: Result { let my_len = self . len () ; let str_len = s . as_bytes () . len () ; if my_len + str_len <= self . capacity () { let remainder = & mut self . data [my_len ..] ; let target = & mut remainder [.. str_len] ; target . copy_from_slice (s . as_bytes ()) ; Ok (()) } else { Err (core :: fmt :: Error) } } }
    };
}

impl_106!();