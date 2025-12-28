macro_rules! deps {
    () => {
        TokenTree!();
    };
}

macro_rules! TtIterSavepoint {
    () => {
        deps!();
        # [derive (Clone , Copy)] pub struct TtIterSavepoint < 'a , S > (& 'a [TokenTree < S >]) ;
    };
}

TtIterSavepoint!();