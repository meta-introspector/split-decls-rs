macro_rules! macro_172 {
    () => {
        decl_derive ! { [PrintAttribute] => # [doc = " Derives `PrintAttribute` for `AttributeKind`."] # [doc = " This macro is pretty specific to `rustc_hir::attrs` and likely not that useful in"] # [doc = " other places. It's deriving something close to `Debug` without printing some extraneous"] # [doc = " things like spans."] print_attribute :: print_attribute }
    };
}

macro_172!();