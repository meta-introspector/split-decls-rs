macro_rules! deps {
    () => {
        Ascii!();
    };
}

macro_rules! impl_7 {
    () => {
        deps!();
        impl < T : AsRef < str > > Ord for Ascii < T > { # [inline] fn cmp (& self , other : & Self) -> Ordering { let self_chars = self . as_ref () . chars () . map (| c | c . to_ascii_lowercase ()) ; let other_chars = other . as_ref () . chars () . map (| c | c . to_ascii_lowercase ()) ; self_chars . cmp (other_chars) } }
    };
}

impl_7!();