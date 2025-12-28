macro_rules! push_integer_62 {
    () => {
        # [doc = " Push a `_`-terminated base 62 integer, using the format"] # [doc = " specified in the RFC as `<base-62-number>`, that is:"] # [doc = " * `x = 0` is encoded as just the `\"_\"` terminator"] # [doc = " * `x > 0` is encoded as `x - 1` in base 62, followed by `\"_\"`,"] # [doc = "   e.g. `1` becomes `\"0_\"`, `62` becomes `\"Z_\"`, etc."] pub (crate) fn push_integer_62 (x : u64 , output : & mut String) { if let Some (x) = x . checked_sub (1) { output . push_str (& x . to_base (62)) ; } output . push ('_') ; }
    };
}

push_integer_62!();