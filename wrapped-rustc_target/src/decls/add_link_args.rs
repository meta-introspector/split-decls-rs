macro_rules! deps {
    () => {
        LinkArgs!();
        LinkerFlavor!();
    };
}

macro_rules! add_link_args {
    () => {
        deps!();
        fn add_link_args (link_args : & mut LinkArgs , flavor : LinkerFlavor , args : & [& 'static str]) { add_link_args_iter (link_args , flavor , args . iter () . copied () . map (Cow :: Borrowed)) }
    };
}

add_link_args!()