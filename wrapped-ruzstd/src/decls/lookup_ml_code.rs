macro_rules! lookup_ml_code {
    () => {
        # [doc = " Look up the provided state value from a match length table predefined"] # [doc = " by the Zstandard reference document. Returns a tuple of (value, number of bits)."] # [doc = ""] # [doc = " <https://github.com/facebook/zstd/blob/dev/doc/zstd_compression_format.md#appendix-a---decoding-tables-for-predefined-codes>"] fn lookup_ml_code (code : u8) -> (u32 , u8) { match code { 0 ..= 31 => (u32 :: from (code) + 3 , 0) , 32 => (35 , 1) , 33 => (37 , 1) , 34 => (39 , 1) , 35 => (41 , 1) , 36 => (43 , 2) , 37 => (47 , 2) , 38 => (51 , 3) , 39 => (59 , 3) , 40 => (67 , 4) , 41 => (83 , 4) , 42 => (99 , 5) , 43 => (131 , 7) , 44 => (259 , 8) , 45 => (515 , 9) , 46 => (1027 , 10) , 47 => (2051 , 11) , 48 => (4099 , 12) , 49 => (8195 , 13) , 50 => (16387 , 14) , 51 => (32771 , 15) , 52 => (65539 , 16) , _ => unreachable ! ("Illegal match length code was: {}" , code) , } }
    };
}

lookup_ml_code!()