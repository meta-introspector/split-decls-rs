macro_rules! MaybeUninitializedPlacesDomain {
    () => {
        # [doc = " There can be many more `MovePathIndex` than there are locals in a MIR body."] # [doc = " We use a mixed bitset to avoid paying too high a memory footprint."] pub type MaybeUninitializedPlacesDomain = MixedBitSet < MovePathIndex > ;
    };
}

MaybeUninitializedPlacesDomain!()