macro_rules! impl_576 {
    () => {
        impl < T > From < T > for PathSegment where T : Into < Ident > , { fn from (ident : T) -> Self { PathSegment { ident : ident . into () , arguments : PathArguments :: None , } } }
    };
}

impl_576!();