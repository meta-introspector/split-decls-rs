// Generated macro for V (struct)
macro_rules! Depcrate_algorithms_myersV {
() => {
// Module: crate::algorithms::myers
// Provides: {"V"}
// Dependencies: {}
# [doc = " `V` contains the endpoints of the furthest reaching `D-paths`. For each"] # [doc = " recorded endpoint `(x,y)` in diagonal `k`, we only need to retain `x` because"] # [doc = " `y` can be computed from `x - k`. In other words, `V` is an array of integers"] # [doc = " where `V[k]` contains the row index of the endpoint of the furthest reaching"] # [doc = " path in diagonal `k`."] # [doc = ""] # [doc = " We can't use a traditional Vec to represent `V` since we use `k` as an index"] # [doc = " and it can take on negative values. So instead `V` is represented as a"] # [doc = " light-weight wrapper around a Vec plus an `offset` which is the maximum value"] # [doc = " `k` can take on in order to map negative `k`'s back to a value >= 0."] # [derive (Debug)] struct V { offset : isize , v : Vec < usize > , }
};
}
