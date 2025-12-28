macro_rules! deps {
    () => {
        TypePrivacyVisitor!();
    };
}

macro_rules! impl_28 {
    () => {
        deps!();
        impl < 'tcx > rustc_ty_utils :: sig_types :: SpannedTypeVisitor < 'tcx > for TypePrivacyVisitor < 'tcx > { type Result = ControlFlow < () > ; fn visit (& mut self , span : Span , value : impl TypeVisitable < TyCtxt < 'tcx > >) -> Self :: Result { self . span = span ; value . visit_with (& mut self . skeleton ()) } }
    };
}

impl_28!()