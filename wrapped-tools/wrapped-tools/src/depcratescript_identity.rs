// Generated macro for SCRIPT_IDENTITY (static)
macro_rules! DepcrateSCRIPT_IDENTITY {
() => {
// Module: crate
// Provides: {"SCRIPT_IDENTITY"}
// Dependencies: {}
static SCRIPT_IDENTITY : LazyLock < Mutex < BTreeMap < PathBuf , u32 > > > = LazyLock :: new (| | Mutex :: new (BTreeMap :: new ())) ;
};
}
