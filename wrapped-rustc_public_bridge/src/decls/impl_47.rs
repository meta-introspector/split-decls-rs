macro_rules! deps {
    () => {
        CompilerCtxt!();
        Bridge!();
        Error!();
    };
}

macro_rules! impl_47 {
    () => {
        deps!();
        impl < 'tcx , B : Bridge > LayoutOfHelpers < 'tcx > for CompilerCtxt < 'tcx , B > { type LayoutOfResult = Result < ty :: layout :: TyAndLayout < 'tcx > , B :: Error > ; # [inline] fn handle_layout_err (& self , err : ty :: layout :: LayoutError < 'tcx > , _span : rustc_span :: Span , ty : Ty < 'tcx > ,) -> B :: Error { B :: Error :: new (format ! ("Failed to get layout for `{ty}`: {err}")) } }
    };
}

impl_47!()