macro_rules! deps {
    () => {
        TraitRef!();
        ExistentialTraitRef!();
        FnSigTys!();
        HostEffectPredicate!();
        TraitPredicate!();
        FnSig!();
        ExistentialPredicate!();
    };
}

macro_rules! macro_220 {
    () => {
        deps!();
        # [cfg (feature = "nightly")] impl_binder_encode_decode ! { ty :: FnSig < I >, ty :: FnSigTys < I >, ty :: TraitPredicate < I >, ty :: ExistentialPredicate < I >, ty :: TraitRef < I >, ty :: ExistentialTraitRef < I >, ty :: HostEffectPredicate < I >, }
    };
}

macro_220!();