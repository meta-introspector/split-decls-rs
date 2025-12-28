macro_rules! OrphanCheckEarlyExit {
    () => {
        enum OrphanCheckEarlyExit < I : Interner , E > { NormalizationFailure (E) , UncoveredTyParam (I :: Ty) , LocalTy (I :: Ty) , }
    };
}

OrphanCheckEarlyExit!()