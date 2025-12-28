macro_rules! deps {
    () => {
        SmolStrBuilderRepr!();
    };
}

macro_rules! impl_62 {
    () => {
        deps!();
        impl Default for SmolStrBuilderRepr { # [inline] fn default () -> Self { SmolStrBuilderRepr :: Inline { buf : [0 ; INLINE_CAP] , len : 0 } } }
    };
}

impl_62!();