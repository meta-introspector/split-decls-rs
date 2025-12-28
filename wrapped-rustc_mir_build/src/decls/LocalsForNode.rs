macro_rules! deps {
    () => {
        ForGuard!();
    };
}

macro_rules! LocalsForNode {
    () => {
        deps!();
        # [derive (Debug)] enum LocalsForNode { # [doc = " In the usual case, a `HirId` for an identifier maps to at most"] # [doc = " one `Local` declaration."] One (Local) , # [doc = " The exceptional case is identifiers in a match arm's pattern"] # [doc = " that are referenced in a guard of that match arm. For these,"] # [doc = " we have `2` Locals."] # [doc = ""] # [doc = " * `for_arm_body` is the Local used in the arm body (which is"] # [doc = "   just like the `One` case above),"] # [doc = ""] # [doc = " * `ref_for_guard` is the Local used in the arm's guard (which"] # [doc = "   is a reference to a temp that is an alias of"] # [doc = "   `for_arm_body`)."] ForGuard { ref_for_guard : Local , for_arm_body : Local } , }
    };
}

LocalsForNode!()