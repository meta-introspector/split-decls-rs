macro_rules! deps {
    () => {
        ReturnCode!();
        InflateConfig!();
    };
}

macro_rules! tests {
    () => {
        deps!();
        # [cfg (test)] mod tests { use super :: * ; # [test] fn uncompress_buffer_overflow () { let mut output = [0 ; 1 << 13] ; let input = [72 , 137 , 58 , 0 , 3 , 39 , 255 , 255 , 255 , 255 , 255 , 14 , 14 , 14 , 14 , 14 , 14 , 14 , 14 , 14 , 14 , 14 , 14 , 184 , 14 , 14 , 14 , 14 , 14 , 14 , 14 , 14 , 14 , 14 , 14 , 14 , 14 , 14 , 14 , 184 , 14 , 14 , 14 , 14 , 14 , 14 , 14 , 63 , 14 , 14 , 14 , 14 , 14 , 14 , 14 , 14 , 184 , 14 , 14 , 255 , 14 , 103 , 14 , 14 , 14 , 14 , 14 , 14 , 61 , 14 , 255 , 255 , 63 , 14 , 14 , 14 , 14 , 14 , 14 , 14 , 14 , 184 , 14 , 14 , 255 , 14 , 14 , 14 , 14 , 14 , 14 , 14 , 14 , 14 , 14 , 14 , 6 , 14 , 14 , 14 , 14 , 14 , 14 , 14 , 14 , 71 , 4 , 137 , 106 ,] ; let config = InflateConfig { window_bits : 15 } ; let (_decompressed , err) = uncompress_slice (& mut output , & input , config) ; assert_eq ! (err , ReturnCode :: DataError) ; } }
    };
}

tests!()