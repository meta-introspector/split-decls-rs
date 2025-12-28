macro_rules! deps {
    () => {
        FingerprintStyle!();
        DepContext!();
        StableHashingContext!();
        DepNode!();
        DepNodeParams!();
    };
}

macro_rules! impl_24 {
    () => {
        deps!();
        impl < Tcx : DepContext , T > DepNodeParams < Tcx > for T where T : for < 'a > HashStable < StableHashingContext < 'a > > + fmt :: Debug , { # [inline (always)] default fn fingerprint_style () -> FingerprintStyle { FingerprintStyle :: Opaque } # [inline (always)] default fn to_fingerprint (& self , tcx : Tcx) -> Fingerprint { tcx . with_stable_hashing_context (| mut hcx | { let mut hasher = StableHasher :: new () ; self . hash_stable (& mut hcx , & mut hasher) ; hasher . finish () }) } # [inline (always)] default fn to_debug_str (& self , tcx : Tcx) -> String { tcx . with_reduced_queries (| | format ! ("{self:?}")) } # [inline (always)] default fn recover (_ : Tcx , _ : & DepNode) -> Option < Self > { None } }
    };
}

impl_24!();