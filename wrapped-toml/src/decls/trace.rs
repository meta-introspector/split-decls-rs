macro_rules! deps {
    () => {
        Style!();
    };
}

macro_rules! trace {
    () => {
        deps!();
        pub (crate) fn trace (text : & str , style : anstyle :: Style) { # ! [allow (unexpected_cfgs)] let depth = DEBUG_DEPTH . depth () ; anstream :: eprintln ! ("{:depth$}{style}{text}{style:#}" , "") ; }
    };
}

trace!()