macro_rules! weakcall {
    () => {
        macro_rules ! weakcall { ($ vis : vis fn $ name : ident ($ ($ arg_name : ident : $ t : ty) ,*) -> $ ret : ty) => ($ vis unsafe fn $ name ($ ($ arg_name : $ t) ,*) -> $ ret { weak ! { fn $ name ($ ($ t) ,*) -> $ ret } if let Some (fun) = $ name . get () { fun ($ ($ arg_name) ,*) } else { libc_errno :: set_errno (libc_errno :: Errno (libc :: ENOSYS)) ; - 1 } }) }
    };
}

weakcall!()