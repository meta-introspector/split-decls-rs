macro_rules! deps {
    () => {
        ArrayVec!();
        Array!();
    };
}

macro_rules! impl_18 {
    () => {
        deps!();
        # [cfg (feature = "arbitrary")] # [cfg_attr (docs_rs , doc (cfg (feature = "arbitrary")))] impl < 'a , A > arbitrary :: Arbitrary < 'a > for ArrayVec < A > where A : Array , A :: Item : arbitrary :: Arbitrary < 'a > , { fn arbitrary (u : & mut arbitrary :: Unstructured < 'a >) -> arbitrary :: Result < Self > { let max_len = A :: CAPACITY . min (u16 :: MAX as usize) as u16 ; let len = u . int_in_range :: < u16 > (0 ..= max_len) ? ; let mut self_ : Self = Default :: default () ; for _ in 0 .. len { self_ . push (u . arbitrary () ?) ; } Ok (self_) } fn size_hint (depth : usize) -> (usize , Option < usize >) { arbitrary :: size_hint :: recursion_guard (depth , | depth | { let max_len = A :: CAPACITY . min (u16 :: MAX as usize) ; let inner = A :: Item :: size_hint (depth) . 1 ; (0 , inner . map (| inner | 2 + max_len * inner)) }) } }
    };
}

impl_18!()