macro_rules! maybe_update_set {
    () => {
        macro_rules ! maybe_update_set { ($ old_pointer : expr , $ new_set : expr) => { { let old_pointer = $ old_pointer ; let new_set = $ new_set ; let old_set : & mut Self = unsafe { & mut * old_pointer } ; if * old_set == new_set { false } else { old_set . clear () ; old_set . extend (new_set) ; return true ; } } } ; }
    };
}

maybe_update_set!();