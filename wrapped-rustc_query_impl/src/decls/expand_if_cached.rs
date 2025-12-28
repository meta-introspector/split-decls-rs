macro_rules! expand_if_cached {
    () => {
        macro_rules ! expand_if_cached { ([] , $ tokens : expr) => { { None } } ; ([(cache) $ ($ rest : tt) *] , $ tokens : expr) => { { Some ($ tokens) } } ; ([$ other : tt $ ($ modifiers : tt) *] , $ tokens : expr) => { expand_if_cached ! ([$ ($ modifiers) *] , $ tokens) } ; }
    };
}

expand_if_cached!()