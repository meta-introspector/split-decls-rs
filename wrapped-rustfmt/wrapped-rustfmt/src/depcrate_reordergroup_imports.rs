// Generated macro for group_imports (function)
macro_rules! Depcrate_reordergroup_imports {
() => {
// Module: crate::reorder
// Provides: {"group_imports"}
// Dependencies: {}
# [doc = " Divides imports into three groups, corresponding to standard, external"] # [doc = " and local imports. Sorts each subgroup."] fn group_imports (uts : Vec < UseTree >) -> Vec < Vec < UseTree > > { let mut std_imports = Vec :: new () ; let mut external_imports = Vec :: new () ; let mut local_imports = Vec :: new () ; for ut in uts . into_iter () { if ut . path . is_empty () { external_imports . push (ut) ; continue ; } match & ut . path [0] . kind { UseSegmentKind :: Ident (id , _) => match id . as_ref () { "std" | "alloc" | "core" => std_imports . push (ut) , _ => external_imports . push (ut) , } , UseSegmentKind :: Slf (_) | UseSegmentKind :: Super (_) | UseSegmentKind :: Crate (_) => { local_imports . push (ut) } UseSegmentKind :: Glob | UseSegmentKind :: List (_) => external_imports . push (ut) , } } vec ! [std_imports , external_imports , local_imports] }
};
}
