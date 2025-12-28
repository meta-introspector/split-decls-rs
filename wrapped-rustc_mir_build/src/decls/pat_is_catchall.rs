macro_rules! pat_is_catchall {
    () => {
        # [doc = " Checks for common cases of \"catchall\" patterns that may not be intended as such."] fn pat_is_catchall (pat : & DeconstructedPat < '_ , '_ >) -> bool { match pat . ctor () { Constructor :: Wildcard => true , Constructor :: Struct | Constructor :: Ref => { pat . iter_fields () . all (| ipat | pat_is_catchall (& ipat . pat)) } _ => false , } }
    };
}

pat_is_catchall!()