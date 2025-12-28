macro_rules! deps {
    () => {
        BodyBuilder!();
    };
}

macro_rules! impl_32 {
    () => {
        deps!();
        impl < 'tcx > BodyBuilder < 'tcx > { pub (crate) fn new (tcx : TyCtxt < 'tcx > , instance : ty :: Instance < 'tcx >) -> Self { let instance = match instance . def { ty :: InstanceKind :: Intrinsic (def_id) => ty :: Instance :: new_raw (def_id , instance . args) , _ => instance , } ; BodyBuilder { tcx , instance } } # [doc = " Build a monomorphic body for a given instance based on the MIR body."] # [doc = ""] # [doc = " All constants are also evaluated."] pub (crate) fn build (mut self) -> mir :: Body < 'tcx > { let body = self . tcx . instance_mir (self . instance . def) . clone () ; let mono_body = if ! self . instance . args . is_empty () || self . tcx . def_kind (self . instance . def_id ()) != DefKind :: AnonConst { let mut mono_body = self . instance . instantiate_mir_and_normalize_erasing_regions (self . tcx , ty :: TypingEnv :: fully_monomorphized () , ty :: EarlyBinder :: bind (body) ,) ; self . visit_body (& mut mono_body) ; mono_body } else { body } ; mono_body } }
    };
}

impl_32!();