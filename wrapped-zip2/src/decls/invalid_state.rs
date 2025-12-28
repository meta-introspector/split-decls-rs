macro_rules! deps {
    () => {
        ZipFileReader!();
    };
}

macro_rules! invalid_state {
    () => {
        deps!();
        # [cold] fn invalid_state < T > () -> io :: Result < T > { Err (io :: Error :: other ("ZipFileReader was in an invalid state")) }
    };
}

invalid_state!();