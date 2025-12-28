macro_rules! deps {
    () => {
        UnwrapLayoutCx!();
    };
}

macro_rules! impl_271 {
    () => {
        deps!();
        impl < 'tcx > LayoutOfHelpers < 'tcx > for UnwrapLayoutCx < 'tcx > { fn handle_layout_err (& self , err : LayoutError < 'tcx > , span : Span , ty : Ty < 'tcx >) -> ! { span_bug ! (span , "`#[rustc_layout(..)]` test resulted in `layout_of({ty}) = Err({err})`" ,) ; } }
    };
}

impl_271!()