macro_rules! BlockState {
    () => {
        # [derive (Debug)] pub (crate) enum BlockState { # [doc = " block not completed, need more input or more output"] NeedMore = 0 , # [doc = " block flush performed"] BlockDone = 1 , # [doc = " finish started, need only more output at next deflate"] FinishStarted = 2 , # [doc = " finish done, accept no more input or output"] FinishDone = 3 , }
    };
}

BlockState!();