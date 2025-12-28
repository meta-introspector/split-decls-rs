macro_rules! pair_lookup_fv_opt {
    () => {
        # [doc = " Extract the value in a pair, returning an option."] # [inline] fn pair_lookup_fv_opt < T > (kv : (u32 , T)) -> Option < T > { Some (kv . 1) }
    };
}

pair_lookup_fv_opt!();