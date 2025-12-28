macro_rules! deps {
    () => {
        Encoding!();
    };
}

macro_rules! UniCase {
    () => {
        deps!();
        # [doc = " Case Insensitive wrapper of strings."] # [derive (Clone , Copy)] pub struct UniCase < S > (Encoding < S >) ;
    };
}

UniCase!();