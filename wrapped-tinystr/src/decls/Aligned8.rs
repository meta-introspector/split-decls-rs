macro_rules! Aligned8 {
    () => {
        # [doc = " Internal helper struct that performs operations on aligned integers."] # [doc = " Supports strings up to 8 bytes long."] # [repr (transparent)] pub struct Aligned8 (u64) ;
    };
}

Aligned8!()