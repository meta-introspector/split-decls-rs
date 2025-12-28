macro_rules! deps {
    () => {
        HasEscapingVarsVisitor!();
        TypeVisitable!();
        HasTypeFlagsVisitor!();
        TypeVisitableExt!();
        Interner!();
        FoundFlags!();
        HasErrorVisitor!();
    };
}

macro_rules! impl_500 {
    () => {
        deps!();
        impl < I : Interner , T : TypeVisitable < I > > TypeVisitableExt < I > for T { fn has_type_flags (& self , flags : TypeFlags) -> bool { let res = self . visit_with (& mut HasTypeFlagsVisitor { flags }) == ControlFlow :: Break (FoundFlags) ; res } fn has_vars_bound_at_or_above (& self , binder : ty :: DebruijnIndex) -> bool { self . visit_with (& mut HasEscapingVarsVisitor { outer_index : binder }) . is_break () } fn error_reported (& self) -> Result < () , I :: ErrorGuaranteed > { if self . references_error () { if let ControlFlow :: Break (guar) = self . visit_with (& mut HasErrorVisitor) { Err (guar) } else { panic ! ("type flags said there was an error, but now there is not") } } else { Ok (()) } } }
    };
}

impl_500!()