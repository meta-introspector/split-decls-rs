macro_rules! deps {
    () => {
        Details!();
        DefaultKind!();
    };
}

macro_rules! s {
    () => {
        deps!();
        macro_rules ! s { ($ name : expr , $ kind : ident) => { Details { signal : $ name , name : stringify ! ($ name) , default_kind : DefaultKind ::$ kind , } } ; }
    };
}

s!();