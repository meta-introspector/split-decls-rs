macro_rules! deps {
    () => {
        ConstValue!();
    };
}

macro_rules! value {
    () => {
        deps!();
        # [doc = " Construct a `ConstValue`."] # [macro_export] macro_rules ! value { ($ ($ json : tt) +) => { $ crate :: value_internal ! ($ ($ json) +) } ; }
    };
}

value!();