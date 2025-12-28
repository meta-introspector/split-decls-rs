macro_rules! deps {
    () => {
        SsoHashMap!();
    };
}

macro_rules! Entry {
    () => {
        deps!();
        # [doc = " A view into a single entry in a map."] pub struct Entry < 'a , K , V > { ssomap : & 'a mut SsoHashMap < K , V > , key : K , }
    };
}

Entry!();