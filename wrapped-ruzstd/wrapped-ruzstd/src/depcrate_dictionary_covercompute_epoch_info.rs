// Generated macro for compute_epoch_info (function)
macro_rules! Depcrate_dictionary_covercompute_epoch_info {
() => {
// Module: crate::dictionary::cover
// Provides: {"compute_epoch_info"}
// Dependencies: {}
# [doc = " Computes the number of epochs and the size of each epoch."] # [doc = ""] # [doc = " Returns a (number of epochs, epoch size) tuple."] # [doc = ""] # [doc = " A translation of `COVER_epoch_info_t COVER_computeEpochs()` from facebook/zstd."] pub fn compute_epoch_info (params : & DictParams , max_dict_size : usize , num_kmers : usize ,) -> (usize , usize) { let min_epoch_size = 10_000 ; let mut num_epochs : usize = usize :: max (1 , max_dict_size / params . segment_size as usize) ; let mut epoch_size : usize = num_kmers / num_epochs ; if epoch_size >= min_epoch_size { assert ! (epoch_size * num_epochs <= num_kmers) ; return (num_epochs , epoch_size) ; } epoch_size = usize :: min (min_epoch_size , num_kmers) ; num_epochs = num_kmers / epoch_size ; (num_epochs , epoch_size) }
};
}
