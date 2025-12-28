macro_rules! deps {
    () => {
        TinyVec!();
        Array!();
    };
}

macro_rules! impl_131 {
    () => {
        deps!();
        # [cfg (feature = "arbitrary")] # [cfg_attr (docs_rs , doc (cfg (feature = "arbitrary")))] impl < 'a , A > arbitrary :: Arbitrary < 'a > for TinyVec < A > where A : Array , A :: Item : arbitrary :: Arbitrary < 'a > , { fn arbitrary (u : & mut arbitrary :: Unstructured < 'a >) -> arbitrary :: Result < Self > { let v = Vec :: arbitrary (u) ? ; let mut tv = TinyVec :: Heap (v) ; tv . shrink_to_fit () ; Ok (tv) } }
    };
}

impl_131!();