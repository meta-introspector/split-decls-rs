macro_rules! deps {
    () => {
        Tlv!();
    };
}

macro_rules! set {
    () => {
        deps!();
        # [doc = " Sets the current thread-local value"] # [inline] pub (crate) fn set (value : Tlv) { TLV . with (| tlv | tlv . set (value . 0)) ; }
    };
}

set!()