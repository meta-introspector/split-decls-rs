macro_rules! deps {
    () => {
        SubdiagnosticKind!();
    };
}

macro_rules! SubdiagnosticVariant {
    () => {
        deps!();
        pub (super) struct SubdiagnosticVariant { pub (super) kind : SubdiagnosticKind , pub (super) slug : Option < Path > , pub (super) no_span : bool , }
    };
}

SubdiagnosticVariant!()