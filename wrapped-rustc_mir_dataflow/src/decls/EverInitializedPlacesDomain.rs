macro_rules! EverInitializedPlacesDomain {
    () => {
        # [doc = " There can be many more `InitIndex` than there are locals in a MIR body."] # [doc = " We use a mixed bitset to avoid paying too high a memory footprint."] pub type EverInitializedPlacesDomain = MixedBitSet < InitIndex > ;
    };
}

EverInitializedPlacesDomain!();