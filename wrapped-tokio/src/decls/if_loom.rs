macro_rules! if_loom {
    () => {
        macro_rules ! if_loom { ($ ($ t : tt) *) => { { # [cfg (loom)] { $ ($ t) * } } } }
    };
}

if_loom!()