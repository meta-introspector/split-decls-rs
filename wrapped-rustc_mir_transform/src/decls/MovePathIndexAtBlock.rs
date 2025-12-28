macro_rules! MovePathIndexAtBlock {
    () => {
        # [doc = " Cache entry of `drop` at a `BasicBlock`"] # [derive (Debug , Clone , Copy)] enum MovePathIndexAtBlock { # [doc = " We know nothing yet"] Unknown , # [doc = " We know that the `drop` here has no effect"] None , # [doc = " We know that the `drop` here will invoke a destructor"] Some (MovePathIndex) , }
    };
}

MovePathIndexAtBlock!()