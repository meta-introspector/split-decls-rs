macro_rules! deps {
    () => {
        RegexCaptures!();
    };
}

macro_rules! RegexMatcher {
    () => {
        deps!();
        pub trait RegexMatcher : Send + Sync { fn new (re : & str) -> Result < Self , String > where Self : Sized ; fn is_match (& self , text : & str) -> bool ; fn captures < 't > (& 't self , text : & 't str) -> Option < Box < dyn RegexCaptures + 't > > ; }
    };
}

RegexMatcher!();