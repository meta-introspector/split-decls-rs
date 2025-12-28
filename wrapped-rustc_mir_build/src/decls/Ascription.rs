macro_rules! Ascription {
    () => {
        # [doc = " Indicates that the type of `source` must be a subtype of the"] # [doc = " user-given type `user_ty`; this is basically a no-op but can"] # [doc = " influence region inference."] # [derive (Clone , Debug)] struct Ascription < 'tcx > { source : Place < 'tcx > , annotation : CanonicalUserTypeAnnotation < 'tcx > , variance : ty :: Variance , }
    };
}

Ascription!();