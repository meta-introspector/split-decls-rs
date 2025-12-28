macro_rules! DepNodeColorMap {
    () => {
        pub (super) struct DepNodeColorMap { values : IndexVec < SerializedDepNodeIndex , AtomicU32 > , sync : bool , }
    };
}

DepNodeColorMap!();