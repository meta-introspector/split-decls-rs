macro_rules! deps {
    () => {
        UniCase!();
    };
}

macro_rules! eq {
    () => {
        deps!();
        # [doc = " Compare two string-like types for case-less equality, using unicode folding."] # [doc = ""] # [doc = " Equivalent to `UniCase::new(left) == UniCase::new(right)`."] # [doc = ""] # [doc = " Note: This will perform a scan for ASCII characters before doing the"] # [doc = " the comparison. See `UniCase` for more information."] # [inline] pub fn eq < S : AsRef < str > + ? Sized > (left : & S , right : & S) -> bool { UniCase :: new (left) == UniCase :: new (right) }
    };
}

eq!();