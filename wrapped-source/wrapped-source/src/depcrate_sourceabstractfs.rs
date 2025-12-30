// Generated macro for AbstractFs (enum)
macro_rules! Depcrate_sourceAbstractFs {
() => {
// Module: crate::source
// Provides: {"AbstractFs"}
// Dependencies: {}
pub (crate) enum AbstractFs { Fs (PathBuf) , Zip (RwLock < Result < ZipData , String > >) , Tar (RwLock < Result < TarArchive , String > >) , Memory (BTreeMap < & 'static str , & 'static [u8] >) , }
};
}
