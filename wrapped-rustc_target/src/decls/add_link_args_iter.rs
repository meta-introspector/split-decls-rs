macro_rules! deps {
    () => {
        LinkArgs!();
        Lld!();
        LinkerFlavor!();
        StaticCow!();
    };
}

macro_rules! add_link_args_iter {
    () => {
        deps!();
        # [doc = " Add arguments for the given flavor and also for its \"twin\" flavors"] # [doc = " that have a compatible command line interface."] fn add_link_args_iter (link_args : & mut LinkArgs , flavor : LinkerFlavor , args : impl Iterator < Item = StaticCow < str > > + Clone ,) { let mut insert = | flavor | link_args . entry (flavor) . or_default () . extend (args . clone ()) ; insert (flavor) ; match flavor { LinkerFlavor :: Gnu (cc , lld) => { assert_eq ! (lld , Lld :: No) ; insert (LinkerFlavor :: Gnu (cc , Lld :: Yes)) ; } LinkerFlavor :: Darwin (cc , lld) => { assert_eq ! (lld , Lld :: No) ; insert (LinkerFlavor :: Darwin (cc , Lld :: Yes)) ; } LinkerFlavor :: Msvc (lld) => { assert_eq ! (lld , Lld :: No) ; insert (LinkerFlavor :: Msvc (Lld :: Yes)) ; } LinkerFlavor :: WasmLld (..) | LinkerFlavor :: Unix (..) | LinkerFlavor :: EmCc | LinkerFlavor :: Bpf | LinkerFlavor :: Llbc | LinkerFlavor :: Ptx => { } } }
    };
}

add_link_args_iter!()