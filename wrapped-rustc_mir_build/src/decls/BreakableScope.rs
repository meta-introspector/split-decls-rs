macro_rules! deps {
    () => {
        DropTree!();
        Scope!();
    };
}

macro_rules! BreakableScope {
    () => {
        deps!();
        # [derive (Debug)] struct BreakableScope < 'tcx > { # [doc = " Region scope of the loop"] region_scope : region :: Scope , # [doc = " The destination of the loop/block expression itself (i.e., where to put"] # [doc = " the result of a `break` or `return` expression)"] break_destination : Place < 'tcx > , # [doc = " Drops that happen on the `break`/`return` path."] break_drops : DropTree , # [doc = " Drops that happen on the `continue` path."] continue_drops : Option < DropTree > , }
    };
}

BreakableScope!();