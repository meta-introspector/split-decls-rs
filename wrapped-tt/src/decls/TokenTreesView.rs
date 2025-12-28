macro_rules! deps {
    () => {
        TokenTree!();
    };
}

macro_rules! TokenTreesView {
    () => {
        deps!();
        # [derive (Clone , Copy)] pub struct TokenTreesView < 'a , S > (& 'a [TokenTree < S >]) ;
    };
}

TokenTreesView!();