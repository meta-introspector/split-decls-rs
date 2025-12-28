macro_rules! deps {
    () => {
        InlineSize!();
        Repr!();
        SmolStr!();
    };
}

macro_rules! impl_4 {
    () => {
        deps!();
        impl Default for SmolStr { # [inline (always)] fn default () -> SmolStr { SmolStr (Repr :: Inline { len : InlineSize :: _V0 , buf : [0 ; INLINE_CAP] }) } }
    };
}

impl_4!()