macro_rules! mph_lookup {
    () => {
        # [doc = " Do a lookup using minimal perfect hashing."] # [doc = ""] # [doc = " The table is stored as a sequence of \"salt\" values, then a sequence of"] # [doc = " values that contain packed key/value pairs. The strategy is to hash twice."] # [doc = " The first hash retrieves a salt value that makes the second hash unique."] # [doc = " The hash function doesn't have to be very good, just good enough that the"] # [doc = " resulting map is unique."] # [inline] pub (crate) fn mph_lookup < KV , V , FK , FV > (x : u32 , salt : & [u16] , kv : & [KV] , fk : FK , fv : FV , default : V ,) -> V where KV : Copy , FK : Fn (KV) -> u32 , FV : Fn (KV) -> V , { let s = salt [my_hash (x , 0 , salt . len ())] as u32 ; let key_val = kv [my_hash (x , s , salt . len ())] ; if x == fk (key_val) { fv (key_val) } else { default } }
    };
}

mph_lookup!()