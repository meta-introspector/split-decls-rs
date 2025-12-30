// Generated macro for Accumulator (trait)
macro_rules! Depcrate_accumulatorAccumulator {
() => {
// Module: crate::accumulator
// Provides: {"Accumulator"}
// Dependencies: {}
# [doc = " Trait implemented on the struct that user annotated with `#[salsa::accumulator]`."] # [doc = " The `Self` type is therefore the types to be accumulated."] pub trait Accumulator : Send + Sync + Any + Sized + UnwindSafe { const DEBUG_NAME : & 'static str ; # [doc = " Accumulate an instance of this in the database for later retrieval."] fn accumulate < Db > (self , db : & Db) where Db : ? Sized + Database ; }
};
}
