macro_rules! deps {
    () => {
        Redactions!();
    };
}

macro_rules! NormalizeRedactions {
    () => {
        deps!();
        struct NormalizeRedactions < 'r > { redactions : & 'r Redactions , }
    };
}

NormalizeRedactions!()