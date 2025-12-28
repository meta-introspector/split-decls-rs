macro_rules! deps {
    () => {
        Ascii!();
    };
}

macro_rules! eq_ascii {
    () => {
        deps!();
        # [doc = " Compare two string-like types for case-less equality, ignoring ASCII case."] # [doc = ""] # [doc = " Equivalent to `Ascii::new(left) == Ascii::new(right)`."] # [inline] pub fn eq_ascii < S : AsRef < str > + ? Sized > (left : & S , right : & S) -> bool { Ascii (left) == Ascii (right) }
    };
}

eq_ascii!()