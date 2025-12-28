macro_rules! deps {
    () => {
        Slab!();
    };
}

macro_rules! unique_iter {
    () => {
        deps!();
        # [test] fn unique_iter () { run_model ("unique_iter" , | | { let mut slab = Arc :: new (Slab :: new ()) ; let s = slab . clone () ; let t1 = thread :: spawn (move | | { s . insert (1) . expect ("insert") ; s . insert (2) . expect ("insert") ; }) ; let s = slab . clone () ; let t2 = thread :: spawn (move | | { s . insert (3) . expect ("insert") ; s . insert (4) . expect ("insert") ; }) ; t1 . join () . expect ("thread 1 should not panic") ; t2 . join () . expect ("thread 2 should not panic") ; let slab = Arc :: get_mut (& mut slab) . expect ("other arcs should be dropped") ; let items : Vec < _ > = slab . unique_iter () . map (| & i | i) . collect () ; assert ! (items . contains (& 1) , "items: {:?}" , items) ; assert ! (items . contains (& 2) , "items: {:?}" , items) ; assert ! (items . contains (& 3) , "items: {:?}" , items) ; assert ! (items . contains (& 4) , "items: {:?}" , items) ; }) ; }
    };
}

unique_iter!();