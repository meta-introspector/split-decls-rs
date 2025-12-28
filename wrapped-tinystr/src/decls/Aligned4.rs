macro_rules! Aligned4 {
    () => {
        # [doc = " Internal helper struct that performs operations on aligned integers."] # [doc = " Supports strings up to 4 bytes long."] # [repr (transparent)] pub struct Aligned4 (u32) ;
    };
}

Aligned4!()