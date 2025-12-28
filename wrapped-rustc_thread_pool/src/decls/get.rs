macro_rules! deps {
    () => {
        Tlv!();
    };
}

macro_rules! get {
    () => {
        deps!();
        # [doc = " Returns the current thread-local value"] # [inline] pub (crate) fn get () -> Tlv { TLV . with (| tlv | Tlv (tlv . get ())) }
    };
}

get!()