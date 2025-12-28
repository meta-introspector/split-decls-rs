macro_rules! ExtendedTimestamp {
    () => {
        # [doc = " extended timestamp, as described in <https://libzip.org/specifications/extrafld.txt>"] # [derive (Debug , Clone)] pub struct ExtendedTimestamp { mod_time : Option < u32 > , ac_time : Option < u32 > , cr_time : Option < u32 > , }
    };
}

ExtendedTimestamp!()