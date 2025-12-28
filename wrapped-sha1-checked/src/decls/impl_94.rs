macro_rules! deps {
    () => {
        Sha1!();
    };
}

macro_rules! impl_94 {
    () => {
        deps!();
        # [cfg (feature = "oid")] impl digest :: const_oid :: AssociatedOid for Sha1 { const OID : digest :: const_oid :: ObjectIdentifier = sha1 :: Sha1 :: OID ; }
    };
}

impl_94!();