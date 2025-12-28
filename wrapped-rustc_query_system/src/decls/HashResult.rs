macro_rules! deps {
    () => {
        StableHashingContext!();
    };
}

macro_rules! HashResult {
    () => {
        deps!();
        pub type HashResult < V > = Option < fn (& mut StableHashingContext < '_ > , & V) -> Fingerprint > ;
    };
}

HashResult!();