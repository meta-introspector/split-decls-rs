macro_rules! dispatch_record {
    () => {
        pub (crate) fn dispatch_record (record : & log :: Record < '_ >) { dispatcher :: get_default (| dispatch | { let filter_meta = record . as_trace () ; if ! dispatch . enabled (& filter_meta) { return ; } let (_ , keys , meta) = loglevel_to_cs (record . level ()) ; let log_module = record . module_path () ; let log_file = record . file () ; let log_line = record . line () ; let module = log_module . as_ref () . map (| s | s as & dyn field :: Value) ; let file = log_file . as_ref () . map (| s | s as & dyn field :: Value) ; let line = log_line . as_ref () . map (| s | s as & dyn field :: Value) ; dispatch . event (& Event :: new (meta , & meta . fields () . value_set (& [(& keys . message , Some (record . args () as & dyn field :: Value)) , (& keys . target , Some (& record . target ())) , (& keys . module , module) , (& keys . file , file) , (& keys . line , line) ,]) ,)) ; }) ; }
    };
}

dispatch_record!()