macro_rules! value_internal_vec {
    () => {
        # [macro_export] # [doc (hidden)] macro_rules ! value_internal_vec { ($ ($ content : tt) *) => { vec ! [$ ($ content) *] } ; }
    };
}

value_internal_vec!();