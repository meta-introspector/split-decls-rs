macro_rules! deps {
    () => {
        DepKind!();
    };
}

macro_rules! Unpacked {
    () => {
        deps!();
        struct Unpacked { len : Option < u32 > , bytes_per_index : usize , kind : DepKind , index : SerializedDepNodeIndex , hash : PackedFingerprint , fingerprint : Fingerprint , }
    };
}

Unpacked!()