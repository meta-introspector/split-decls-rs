macro_rules! stringify_punct {
    () => {
        # [doc (hidden)] # [macro_export] macro_rules ! stringify_punct { ($ ($ tt : tt) +) => { $ crate :: __private :: concat ! ($ ($ crate :: __private :: stringify ! ($ tt)) ,+) } ; }
    };
}

stringify_punct!();