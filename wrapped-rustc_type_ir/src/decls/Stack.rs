macro_rules! deps {
    () => {
        Cx!();
        StackEntry!();
    };
}

macro_rules! Stack {
    () => {
        deps!();
        # [doc = " The stack of goals currently being computed."] # [doc = ""] # [doc = " An element is *deeper* in the stack if its index is *lower*."] # [doc = ""] # [doc = " Only the last entry of the stack is mutable. All other entries get"] # [doc = " lazily updated in `update_parent_goal`."] # [derive_where (Default ; X : Cx)] pub (super) struct Stack < X : Cx > { entries : IndexVec < StackDepth , StackEntry < X > > , }
    };
}

Stack!();