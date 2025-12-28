macro_rules! pair_lookup_fk {
    () => {
        # [doc = " Extract the key in a pair."] # [inline] fn pair_lookup_fk < T > (kv : (u32 , T)) -> u32 { kv . 0 }
    };
}

pair_lookup_fk!();