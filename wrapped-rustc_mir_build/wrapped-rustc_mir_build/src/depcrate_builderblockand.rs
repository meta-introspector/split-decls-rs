// Generated macro for BlockAnd (struct)
macro_rules! Depcrate_builderBlockAnd {
() => {
// Module: crate::builder
// Provides: {"BlockAnd"}
// Dependencies: {}
# [doc = " The `BlockAnd` \"monad\" packages up the new basic block along with a"] # [doc = " produced value (sometimes just unit, of course). The `unpack!`"] # [doc = " macro (and methods below) makes working with `BlockAnd` much more"] # [doc = " convenient."] # [must_use = "if you don't use one of these results, you're leaving a dangling edge"] struct BlockAnd < T > (BasicBlock , T) ;
};
}
