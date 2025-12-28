macro_rules! deps {
    () => {
        Opaque!();
    };
}

macro_rules! opaque {
    () => {
        deps!();
        pub fn opaque < T : Debug > (value : & T) -> Opaque { Opaque (format ! ("{value:?}")) }
    };
}

opaque!();