// Generated macro for ExtendedData (enum)
macro_rules! DepcrateExtendedData {
() => {
// Module: crate
// Provides: {"ExtendedData"}
// Dependencies: {}
# [doc = " How to handle extended data streams, such as stderr"] # [derive (Copy , Clone , Debug)] pub enum ExtendedData { # [doc = " Queue extended data for eventual reading"] Normal = raw :: LIBSSH2_CHANNEL_EXTENDED_DATA_NORMAL as isize , # [doc = " Treat extended data and ordinary data the same. Merge all substreams such that calls to"] # [doc = " read will pull from all substreams on a first-in/first-out basis."] Merge = raw :: LIBSSH2_CHANNEL_EXTENDED_DATA_MERGE as isize , # [doc = " Discard all extended data as it arrives."] Ignore = raw :: LIBSSH2_CHANNEL_EXTENDED_DATA_IGNORE as isize , }
};
}
