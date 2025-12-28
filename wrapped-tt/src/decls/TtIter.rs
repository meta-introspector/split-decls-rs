macro_rules! deps {
    () => {
        TokenTree!();
    };
}

macro_rules! TtIter {
    () => {
        deps!();
        # [derive (Clone)] pub struct TtIter < 'a , S > { inner : std :: slice :: Iter < 'a , TokenTree < S > > , }
    };
}

TtIter!()