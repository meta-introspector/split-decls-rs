macro_rules! core_internal {
    () => {
        # [doc (hidden)] macro_rules ! core_internal { ($ names : ident) => { let unlocks : Vec < _ > = $ names . into_iter () . map (| name | { check_new_key (name) ; global_locks () . get (name) . expect ("key to be set") . get () . clone () }) . collect () ; let _guards : Vec < _ > = unlocks . iter () . map (| unlock | unlock . lock ()) . collect () ; } ; }
    };
}

core_internal!()