macro_rules! deps {
    () => {
        Directory!();
    };
}

macro_rules! tokenize_path {
    () => {
        deps!();
        macro_rules ! tokenize_path { ([$ (($ ($ component : tt) +)) *] [$ ($ cur : tt) +] /) => { crate :: directory :: Directory :: new (tokenize_path ! ([$ (($ ($ component) +)) *] [$ ($ cur) +])) } ; ([$ (($ ($ component : tt) +)) *] [$ ($ cur : tt) +] / $ ($ rest : tt) +) => { tokenize_path ! ([$ (($ ($ component) +)) * ($ ($ cur) +)] [] $ ($ rest) +) } ; ([$ (($ ($ component : tt) +)) *] [$ ($ cur : tt) *] $ first : tt $ ($ rest : tt) *) => { tokenize_path ! ([$ (($ ($ component) +)) *] [$ ($ cur) * $ first] $ ($ rest) *) } ; ([$ (($ ($ component : tt) +)) *] [$ ($ cur : tt) +]) => { tokenize_path ! ([$ (($ ($ component) +)) * ($ ($ cur) +)]) } ; ([$ (($ ($ component : tt) +)) *]) => { { let mut path = std :: path :: PathBuf :: new () ; $ (path . push (& ($ ($ component) +)) ;) * path } } ; }
    };
}

tokenize_path!();