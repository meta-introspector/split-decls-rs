macro_rules! debug_assert_eq_in_drop {
    () => {
        macro_rules ! debug_assert_eq_in_drop { ($ this : expr , $ that : expr) => { debug_assert_eq_in_drop ! (@ inner $ this , $ that , "") } ; ($ this : expr , $ that : expr , $ ($ arg : tt) +) => { debug_assert_eq_in_drop ! (@ inner $ this , $ that , format_args ! (": {}" , format_args ! ($ ($ arg) +))) } ; (@ inner $ this : expr , $ that : expr , $ msg : expr) => { if cfg ! (debug_assertions) { if $ this != $ that { panic_in_drop ! ("assertion failed ({} == {})\n  left: `{:?}`,\n right: `{:?}`{}" , stringify ! ($ this) , stringify ! ($ that) , $ this , $ that , $ msg ,) } } } }
    };
}

debug_assert_eq_in_drop!()