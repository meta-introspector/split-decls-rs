macro_rules! bitflags_bits {
    () => {
        # [doc = " Return a [`bitcast`] of the value of `$x.bits()`, where `$x` is a"] # [doc = " `bitflags` type."] macro_rules ! bitflags_bits { ($ x : expr) => { { bitcast ! ($ x . bits ()) } } ; }
    };
}

bitflags_bits!()