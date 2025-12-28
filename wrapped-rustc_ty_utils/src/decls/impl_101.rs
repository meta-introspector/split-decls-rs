macro_rules! deps {
    () => {
        OpaqueTypeCollector!();
        SpannedTypeVisitor!();
    };
}

macro_rules! impl_101 {
    () => {
        deps!();
        impl < 'tcx > super :: sig_types :: SpannedTypeVisitor < 'tcx > for OpaqueTypeCollector < 'tcx > { # [instrument (skip (self) , ret , level = "trace")] fn visit (& mut self , span : Span , value : impl TypeVisitable < TyCtxt < 'tcx > >) { self . visit_spanned (span , value) ; } }
    };
}

impl_101!();