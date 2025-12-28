macro_rules! deps {
    () => {
        Deps!();
        DepGraph!();
        MarkFrame!();
    };
}

macro_rules! print_markframe_trace {
    () => {
        deps!();
        # [inline (never)] # [cold] pub (crate) fn print_markframe_trace < D : Deps > (graph : & DepGraph < D > , frame : Option < & MarkFrame < '_ > >) { let data = graph . data . as_ref () . unwrap () ; eprintln ! ("there was a panic while trying to force a dep node") ; eprintln ! ("try_mark_green dep node stack:") ; let mut i = 0 ; let mut current = frame ; while let Some (frame) = current { let node = data . previous . index_to_node (frame . index) ; eprintln ! ("#{i} {node:?}") ; current = frame . parent ; i += 1 ; } eprintln ! ("end of try_mark_green dep node stack") ; }
    };
}

print_markframe_trace!()