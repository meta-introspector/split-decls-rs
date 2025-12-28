macro_rules! LocationMap {
    () => {
        # [derive (Debug)] pub struct LocationMap < T > { # [doc = " Location-indexed (BasicBlock for outer index, index within BB"] # [doc = " for inner index) map."] pub (crate) map : IndexVec < BasicBlock , Vec < T > > , }
    };
}

LocationMap!();