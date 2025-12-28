macro_rules! deps {
    () => {
        Chunk!();
    };
}

macro_rules! impl_121 {
    () => {
        deps!();
        impl < 'a , A , const N : usize > Arbitrary < 'a > for Chunk < A , N > where A : Arbitrary < 'a > , BitsImpl < N > : Bits , { fn arbitrary (u : & mut Unstructured < 'a >) -> Result < Self > { u . arbitrary_iter () ? . take (Self :: CAPACITY) . collect () } fn arbitrary_take_rest (u : Unstructured < 'a >) -> Result < Self > { u . arbitrary_take_rest_iter () ? . take (Self :: CAPACITY) . collect () } fn size_hint (depth : usize) -> (usize , Option < usize >) { size_hint :: recursion_guard (depth , | depth | { let (_ , upper) = A :: size_hint (depth) ; (0 , upper . map (| upper | upper * Self :: CAPACITY)) }) } }
    };
}

impl_121!()