// Generated macro for WriterInnerLock (enum)
macro_rules! DepcrateWriterInnerLock {
() => {
// Module: crate
// Provides: {"WriterInnerLock"}
// Dependencies: {}
# [doc = " WriterInnerLock is a (limited) generic representation of a writer. It is"] # [doc = " limited because W should only ever be stdout/stderr on Windows."] # [derive (Debug)] enum WriterInnerLock < 'a , W > { NoColor (NoColor < W >) , Ansi (Ansi < W >) , # [doc = " What a gross hack. On Windows, we need to specify a lifetime for the"] # [doc = " console when in a locked state, but obviously don't need to do that"] # [doc = " on Unix, which makes the `'a` unused. To satisfy the compiler, we need"] # [doc = " a PhantomData."] # [allow (dead_code)] Unreachable (:: std :: marker :: PhantomData < & 'a () >) , # [cfg (windows)] Windows { wtr : W , console : MutexGuard < 'a , wincon :: Console > , } , }
};
}
