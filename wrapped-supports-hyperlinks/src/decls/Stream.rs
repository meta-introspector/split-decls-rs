macro_rules! Stream {
    () => {
        # [doc = " possible stream sources"] # [derive (Clone , Copy , Debug)] pub enum Stream { Stdout , Stderr , }
    };
}

Stream!()