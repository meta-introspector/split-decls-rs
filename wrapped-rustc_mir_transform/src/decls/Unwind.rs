macro_rules! Unwind {
    () => {
        # [doc = " Describes if unwinding is necessary and where to unwind to if a panic occurs."] # [derive (Copy , Clone , Debug)] pub (crate) enum Unwind { # [doc = " Unwind to this block."] To (BasicBlock) , # [doc = " Already in an unwind path, any panic will cause an abort."] InCleanup , }
    };
}

Unwind!()