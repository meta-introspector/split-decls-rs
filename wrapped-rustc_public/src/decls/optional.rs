macro_rules! optional {
    () => {
        # [doc = " Optionally include an ident. This is needed due to macro hygiene."] # [macro_export] # [doc (hidden)] macro_rules ! optional { (with_tcx $ ident : ident) => { $ ident } ; }
    };
}

optional!()