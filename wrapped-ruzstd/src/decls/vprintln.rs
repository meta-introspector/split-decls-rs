macro_rules! vprintln {
    () => {
        macro_rules ! vprintln { ($ ($ x : expr) ,*) => { # [cfg (feature = "std")] if crate :: VERBOSE { std :: println ! ($ ($ x) ,*) ; } } }
    };
}

vprintln!();