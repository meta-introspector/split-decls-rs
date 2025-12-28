macro_rules! deps {
    () => {
        TokenTree!();
    };
}

macro_rules! TopSubtree {
    () => {
        deps!();
        # [derive (Clone , PartialEq , Eq , Hash)] pub struct TopSubtree < S > (pub Box < [TokenTree < S >] >) ;
    };
}

TopSubtree!();