macro_rules! deps {
    () => {
        Object!();
    };
}

macro_rules! Callsite {
    () => {
        deps!();
        struct Callsite { tid : usize , name : String , target : String , file : Option < & 'static str > , line : Option < u32 > , args : Option < Arc < Object > > , }
    };
}

Callsite!()