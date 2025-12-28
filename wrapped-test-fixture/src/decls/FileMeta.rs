macro_rules! deps {
    () => {
        SourceRootKind!();
    };
}

macro_rules! FileMeta {
    () => {
        deps!();
        # [derive (Debug)] struct FileMeta { path : String , krate : Option < (String , CrateOrigin , Option < String >) > , deps : Vec < String > , extern_prelude : Option < Vec < String > > , cfg : CfgOptions , edition : Edition , env : Env , introduce_new_source_root : Option < SourceRootKind > , }
    };
}

FileMeta!()