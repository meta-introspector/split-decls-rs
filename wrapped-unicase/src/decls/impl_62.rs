macro_rules! deps {
    () => {
        Unicode!();
        UniCase!();
        Ascii!();
        Encoding!();
    };
}

macro_rules! impl_62 {
    () => {
        deps!();
        impl < T : AsRef < str > > Ord for UniCase < T > { # [inline] fn cmp (& self , other : & Self) -> Ordering { match (& self . 0 , & other . 0) { (& Encoding :: Ascii (ref x) , & Encoding :: Ascii (ref y)) => x . cmp (y) , (& Encoding :: Unicode (ref x) , & Encoding :: Unicode (ref y)) => x . cmp (y) , (& Encoding :: Ascii (ref x) , & Encoding :: Unicode (ref y)) => { Unicode (x . as_ref ()) . cmp (& Unicode (y . 0 . as_ref ())) } (& Encoding :: Unicode (ref x) , & Encoding :: Ascii (ref y)) => { Unicode (x . 0 . as_ref ()) . cmp (& Unicode (y . as_ref ())) } } } }
    };
}

impl_62!()