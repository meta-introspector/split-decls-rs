macro_rules! deps {
    () => {
        SsoHashSet!();
        SsoHashMap!();
    };
}

macro_rules! entry_to_key {
    () => {
        deps!();
        # [doc = " Adapter function used to return"] # [doc = " result if SsoHashMap functions into"] # [doc = " result SsoHashSet should return."] # [inline (always)] fn entry_to_key < K , V > ((k , _v) : (K , V)) -> K { k }
    };
}

entry_to_key!();