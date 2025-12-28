macro_rules! deps {
    () => {
        IsFirstInputType!();
        UncoveredTyParams!();
    };
}

macro_rules! OrphanCheckErr {
    () => {
        deps!();
        # [derive_where (Debug ; I : Interner , T : Debug)] pub enum OrphanCheckErr < I : Interner , T > { NonLocalInputType (Vec < (I :: Ty , IsFirstInputType) >) , UncoveredTyParams (UncoveredTyParams < I , T >) , }
    };
}

OrphanCheckErr!();