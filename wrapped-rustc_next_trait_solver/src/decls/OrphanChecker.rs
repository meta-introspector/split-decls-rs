macro_rules! deps {
    () => {
        InCrate!();
        IsFirstInputType!();
    };
}

macro_rules! OrphanChecker {
    () => {
        deps!();
        struct OrphanChecker < 'a , Infcx , I : Interner , F > { infcx : & 'a Infcx , in_crate : InCrate , in_self_ty : bool , lazily_normalize_ty : F , # [doc = " Ignore orphan check failures and exclusively search for the first local type."] search_first_local_ty : bool , non_local_tys : Vec < (I :: Ty , IsFirstInputType) > , }
    };
}

OrphanChecker!()