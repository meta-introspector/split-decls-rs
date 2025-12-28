macro_rules! deps {
    () => {
        Inner!();
    };
}

macro_rules! PanicMsgSnippet {
    () => {
        deps!();
        struct PanicMsgSnippet < 'a > (& 'a Inner) ;
    };
}

PanicMsgSnippet!()