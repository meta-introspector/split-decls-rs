macro_rules! deps {
    () => {
        Speculative!();
        ParseBuffer!();
        Unexpected!();
        Cursor!();
    };
}

macro_rules! impl_491 {
    () => {
        deps!();
        impl < 'a > Speculative for ParseBuffer < 'a > { fn advance_to (& self , fork : & Self) { if ! crate :: buffer :: same_scope (self . cursor () , fork . cursor ()) { panic ! ("fork was not derived from the advancing parse stream") ; } let (self_unexp , self_sp) = inner_unexpected (self) ; let (fork_unexp , fork_sp) = inner_unexpected (fork) ; if ! Rc :: ptr_eq (& self_unexp , & fork_unexp) { match (fork_sp , self_sp) { (Some ((span , delimiter)) , None) => { self_unexp . set (Unexpected :: Some (span , delimiter)) ; } (None , None) => { fork_unexp . set (Unexpected :: Chain (self_unexp)) ; fork . unexpected . set (Some (Rc :: new (Cell :: new (Unexpected :: None)))) ; } (_ , Some (_)) => { } } } self . cell . set (unsafe { mem :: transmute :: < Cursor , Cursor < 'static > > (fork . cursor ()) }) ; } }
    };
}

impl_491!();