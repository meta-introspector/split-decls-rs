macro_rules! deps {
    () => {
        Encoding!();
        Ascii!();
        Unicode!();
    };
}

macro_rules! inner {
    () => {
        deps!();
        macro_rules ! inner { (mut $ e : expr) => { { match & mut $ e { & mut Encoding :: Ascii (ref mut s) => & mut s . 0 , & mut Encoding :: Unicode (ref mut s) => & mut s . 0 , } } } ; ($ e : expr) => { { match &$ e { & Encoding :: Ascii (ref s) => & s . 0 , & Encoding :: Unicode (ref s) => & s . 0 , } } } ; }
    };
}

inner!()