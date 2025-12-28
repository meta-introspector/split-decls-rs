macro_rules! deps {
    () => {
        LinkArgs!();
        TargetOptions!();
        Lld!();
        LinkerFlavor!();
    };
}

macro_rules! impl_540 {
    () => {
        deps!();
        impl TargetOptions { fn link_args (flavor : LinkerFlavor , args : & [& 'static str]) -> LinkArgs { let mut link_args = LinkArgs :: new () ; add_link_args (& mut link_args , flavor , args) ; link_args } fn add_pre_link_args (& mut self , flavor : LinkerFlavor , args : & [& 'static str]) { add_link_args (& mut self . pre_link_args , flavor , args) ; } fn update_from_cli (& mut self) { self . linker_flavor = LinkerFlavor :: from_cli_json (self . linker_flavor_json , self . lld_flavor_json , self . linker_is_gnu_json ,) ; for (args , args_json) in [(& mut self . pre_link_args , & self . pre_link_args_json) , (& mut self . late_link_args , & self . late_link_args_json) , (& mut self . late_link_args_dynamic , & self . late_link_args_dynamic_json) , (& mut self . late_link_args_static , & self . late_link_args_static_json) , (& mut self . post_link_args , & self . post_link_args_json) ,] { args . clear () ; for (flavor , args_json) in args_json { let linker_flavor = self . linker_flavor . with_cli_hints (* flavor) ; let linker_flavor = match linker_flavor { LinkerFlavor :: Gnu (cc , _) => LinkerFlavor :: Gnu (cc , Lld :: No) , LinkerFlavor :: Darwin (cc , _) => LinkerFlavor :: Darwin (cc , Lld :: No) , LinkerFlavor :: Msvc (_) => LinkerFlavor :: Msvc (Lld :: No) , _ => linker_flavor , } ; if ! args . contains_key (& linker_flavor) { add_link_args_iter (args , linker_flavor , args_json . iter () . cloned ()) ; } } } } fn update_to_cli (& mut self) { self . linker_flavor_json = self . linker_flavor . to_cli_counterpart () ; self . lld_flavor_json = self . linker_flavor . lld_flavor () ; self . linker_is_gnu_json = self . linker_flavor . is_gnu () ; for (args , args_json) in [(& self . pre_link_args , & mut self . pre_link_args_json) , (& self . late_link_args , & mut self . late_link_args_json) , (& self . late_link_args_dynamic , & mut self . late_link_args_dynamic_json) , (& self . late_link_args_static , & mut self . late_link_args_static_json) , (& self . post_link_args , & mut self . post_link_args_json) ,] { * args_json = args . iter () . map (| (flavor , args) | (flavor . to_cli_counterpart () , args . clone ())) . collect () ; } } }
    };
}

impl_540!()