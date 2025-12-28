macro_rules! deps {
    () => {
        Deps!();
        WorkProductMap!();
        WorkProduct!();
        DepNodeColor!();
        DepNode!();
        WorkProductId!();
        QueryContext!();
        DepGraph!();
    };
}

macro_rules! impl_54 {
    () => {
        deps!();
        impl < D : Deps > DepGraph < D > { # [doc = " Checks whether a previous work product exists for `v` and, if"] # [doc = " so, return the path that leads to it. Used to skip doing work."] pub fn previous_work_product (& self , v : & WorkProductId) -> Option < WorkProduct > { self . data . as_ref () . and_then (| data | data . previous_work_products . get (v) . cloned ()) } # [doc = " Access the map of work-products created during the cached run. Only"] # [doc = " used during saving of the dep-graph."] pub fn previous_work_products (& self) -> & WorkProductMap { & self . data . as_ref () . unwrap () . previous_work_products } pub fn debug_was_loaded_from_disk (& self , dep_node : DepNode) -> bool { self . data . as_ref () . unwrap () . debug_loaded_from_disk . lock () . contains (& dep_node) } # [cfg (debug_assertions)] # [inline (always)] pub (crate) fn register_dep_node_debug_str < F > (& self , dep_node : DepNode , debug_str_gen : F) where F : FnOnce () -> String , { let dep_node_debug = & self . data . as_ref () . unwrap () . dep_node_debug ; if dep_node_debug . borrow () . contains_key (& dep_node) { return ; } let debug_str = self . with_ignore (debug_str_gen) ; dep_node_debug . borrow_mut () . insert (dep_node , debug_str) ; } pub fn dep_node_debug_str (& self , dep_node : DepNode) -> Option < String > { self . data . as_ref () ? . dep_node_debug . borrow () . get (& dep_node) . cloned () } fn node_color (& self , dep_node : & DepNode) -> Option < DepNodeColor > { if let Some (ref data) = self . data { return data . node_color (dep_node) ; } None } pub fn try_mark_green < Qcx : QueryContext < Deps = D > > (& self , qcx : Qcx , dep_node : & DepNode ,) -> Option < (SerializedDepNodeIndex , DepNodeIndex) > { self . data () . and_then (| data | data . try_mark_green (qcx , dep_node)) } }
    };
}

impl_54!()