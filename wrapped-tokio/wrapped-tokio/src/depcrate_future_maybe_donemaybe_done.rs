// Generated macro for maybe_done (function)
macro_rules! Depcrate_future_maybe_donemaybe_done {
() => {
// Module: crate::future::maybe_done
// Provides: {"maybe_done"}
// Dependencies: {}
# [doc = " Wraps a future into a `MaybeDone`."] pub fn maybe_done < F : IntoFuture > (future : F) -> MaybeDone < F :: IntoFuture > { MaybeDone :: Future { future : future . into_future () , } }
};
}
