// Generated macro for SeekWrite (trait)
macro_rules! Depcrate_builderSeekWrite {
() => {
// Module: crate::builder
// Provides: {"SeekWrite"}
// Dependencies: {}
trait SeekWrite : Write + Seek { fn as_write (& mut self) -> & mut dyn Write ; }
};
}
