macro_rules! rtry {
    () => {
        macro_rules ! rtry { ($ e : expr) => { match $ e { e @ Representability :: Infinite (_) => return e , Representability :: Representable => { } } } ; }
    };
}

rtry!();