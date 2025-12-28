macro_rules! deps {
    () => {
        TokenTree!();
    };
}

macro_rules! SubtreeView {
    () => {
        deps!();
        # [derive (Clone , Copy)] pub struct SubtreeView < 'a , S > (& 'a [TokenTree < S >]) ;
    };
}

SubtreeView!();