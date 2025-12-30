// Generated macro for Collector (struct)
macro_rules! Depcrate_collectorCollector {
() => {
// Module: crate::collector
// Provides: {"Collector"}
// Dependencies: {}
# [doc = " [`Collector`] is a garbage collector that reclaims thread-locally unreachable instances"] # [doc = " when they are globally unreachable."] # [derive (Debug , Default)] # [repr (align (128))] pub (super) struct Collector { state : AtomicU8 , announcement : Epoch , next_epoch_update : u8 , has_garbage : bool , num_readers : u32 , previous_instance_link : Option < NonNull < dyn Collectible > > , current_instance_link : Option < NonNull < dyn Collectible > > , next_instance_link : Option < NonNull < dyn Collectible > > , next_link : AtomicPtr < Collector > , link : Link , }
};
}
