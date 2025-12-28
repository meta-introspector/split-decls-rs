macro_rules! custom_punctuation_repr {
    () => {
        # [doc (hidden)] # [macro_export] macro_rules ! custom_punctuation_repr { ($ ($ tt : tt) +) => { [$ crate :: __private :: Span ; 0 $ (+ $ crate :: custom_punctuation_len ! (lenient , $ tt)) +] } ; }
    };
}

custom_punctuation_repr!();