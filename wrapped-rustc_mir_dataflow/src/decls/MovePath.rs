macro_rules! MovePath {
    () => {
        # [doc = " `MovePath` is a canonicalized representation of a path that is"] # [doc = " moved or assigned to."] # [doc = ""] # [doc = " It follows a tree structure."] # [doc = ""] # [doc = " Given `struct X { m: M, n: N }` and `x: X`, moves like `drop x.m;`"] # [doc = " move *out* of the place `x.m`."] # [doc = ""] # [doc = " The MovePaths representing `x.m` and `x.n` are siblings (that is,"] # [doc = " one of them will link to the other via the `next_sibling` field,"] # [doc = " and the other will have no entry in its `next_sibling` field), and"] # [doc = " they both have the MovePath representing `x` as their parent."] # [derive (Clone)] pub struct MovePath < 'tcx > { pub next_sibling : Option < MovePathIndex > , pub first_child : Option < MovePathIndex > , pub parent : Option < MovePathIndex > , pub place : Place < 'tcx > , }
    };
}

MovePath!();