macro_rules! deps {
    () => {
        PathKind!();
        NestedGoals!();
        Cx!();
    };
}

macro_rules! impl_158 {
    () => {
        deps!();
        impl < X : Cx > NestedGoals < X > { fn is_empty (& self) -> bool { self . nested_goals . is_empty () } fn insert (& mut self , input : X :: Input , paths_to_nested : PathsToNested) { match self . nested_goals . entry (input) { Entry :: Occupied (mut entry) => * entry . get_mut () |= paths_to_nested , Entry :: Vacant (entry) => drop (entry . insert (paths_to_nested)) , } } # [doc = " Adds the nested goals of a nested goal, given that the path `step_kind` from this goal"] # [doc = " to the parent goal."] # [doc = ""] # [doc = " If the path from this goal to the nested goal is inductive, the paths from this goal"] # [doc = " to all nested goals of that nested goal are also inductive. Otherwise the paths are"] # [doc = " the same as for the child."] fn extend_from_child (& mut self , step_kind : PathKind , nested_goals : & NestedGoals < X >) { # [allow (rustc :: potential_query_instability)] for (input , paths_to_nested) in nested_goals . iter () { let paths_to_nested = paths_to_nested . extend_with (step_kind) ; self . insert (input , paths_to_nested) ; } } # [cfg_attr (feature = "nightly" , rustc_lint_query_instability)] # [allow (rustc :: potential_query_instability)] fn iter (& self) -> impl Iterator < Item = (X :: Input , PathsToNested) > + '_ { self . nested_goals . iter () . map (| (i , p) | (* i , * p)) } fn contains (& self , input : X :: Input) -> bool { self . nested_goals . contains_key (& input) } }
    };
}

impl_158!();