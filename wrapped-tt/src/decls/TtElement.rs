macro_rules! deps {
    () => {
        Leaf!();
        TtIter!();
        Subtree!();
    };
}

macro_rules! TtElement {
    () => {
        deps!();
        # [derive (Clone)] pub enum TtElement < 'a , S > { Leaf (& 'a Leaf < S >) , Subtree (& 'a Subtree < S > , TtIter < 'a , S >) , }
    };
}

TtElement!()