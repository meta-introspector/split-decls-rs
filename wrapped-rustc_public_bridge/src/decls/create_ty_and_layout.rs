macro_rules! deps {
    () => {
        TypingEnvHelpers!();
        Bridge!();
        CompilerCtxt!();
    };
}

macro_rules! create_ty_and_layout {
    () => {
        deps!();
        pub fn create_ty_and_layout < 'tcx , B : Bridge > (cx : & CompilerCtxt < 'tcx , B > , ty : Ty < 'tcx > ,) -> Result < TyAndLayout < 'tcx , Ty < 'tcx > > , & 'tcx layout :: LayoutError < 'tcx > > { use crate :: context :: TypingEnvHelpers ; cx . tcx . layout_of (cx . fully_monomorphized () . as_query_input (ty)) }
    };
}

create_ty_and_layout!();