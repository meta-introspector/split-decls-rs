macro_rules! compare_tail {
    () => {
        macro_rules ! compare_tail { ($ slice : ident , $ bytes : expr) => { compare_tail ! ($ slice , $ bytes , 1) } ; ($ slice : ident , $ bytes : expr , $ from : expr) => { compare_tail ! ($ slice , $ bytes . len () + $ from , $ bytes , $ from) } ; ($ slice : ident , $ len : expr , $ bytes : expr , $ from : expr) => { $ slice . len () >= $ len && $ slice [$ from ..$ from + $ bytes . len ()] == $ bytes } ; }
    };
}

compare_tail!();