macro_rules! deps {
    () => {
        Scope!();
        DropTree!();
    };
}

macro_rules! IfThenScope {
    () => {
        deps!();
        # [derive (Debug)] struct IfThenScope { # [doc = " The if-then scope or arm scope"] region_scope : region :: Scope , # [doc = " Drops that happen on the `else` path."] else_drops : DropTree , }
    };
}

IfThenScope!();