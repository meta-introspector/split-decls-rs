macro_rules! deps {
    () => {
        JoinedArgs!();
    };
}

macro_rules! impl_21 {
    () => {
        deps!();
        impl std :: str :: FromStr for JoinedArgs { type Err = std :: convert :: Infallible ; fn from_str (s : & str) -> Result < Self , Self :: Err > { let inner = shlex :: Shlex :: new (s) . collect () ; Ok (Self { inner }) } }
    };
}

impl_21!();