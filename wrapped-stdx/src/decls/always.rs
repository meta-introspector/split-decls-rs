macro_rules! always {
    () => {
        # [doc = " Asserts that the condition is always true and returns its actual value."] # [doc = ""] # [doc = " If the condition is true does nothing and and evaluates to true."] # [doc = ""] # [doc = " If the condition is false:"] # [doc = " * panics if `force` feature or `debug_assertions` are enabled,"] # [doc = " * logs an error if the `tracing` feature is enabled,"] # [doc = " * evaluates to false."] # [doc = ""] # [doc = " Accepts `format!` style arguments."] # [macro_export] macro_rules ! always { ($ cond : expr) => { $ crate :: always ! ($ cond , "assertion failed: {}" , stringify ! ($ cond)) } ; ($ cond : expr , $ fmt : literal $ ($ arg : tt) *) => { { let cond = $ cond ; if cfg ! (debug_assertions) || $ crate :: assert :: __FORCE { assert ! (cond , $ fmt $ ($ arg) *) ; } if ! cond { $ crate :: assert :: __tracing_error ! ($ fmt $ ($ arg) *) ; } cond } } ; }
    };
}

always!();