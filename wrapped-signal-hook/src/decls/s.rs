macro_rules! deps {
    () => {
        DefaultKind!();
        Details!();
    };
}

macro_rules! s {
    () => {
        deps!();
        macro_rules ! s { ($ name : expr , $ kind : ident) => { Details { signal : $ name , name : stringify ! ($ name) , default_kind : DefaultKind ::$ kind , } } ; }
    };
}

s!()