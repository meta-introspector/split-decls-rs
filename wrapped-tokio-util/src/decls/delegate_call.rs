macro_rules! delegate_call {
    () => {
        # [doc = " A small helper macro which reduces amount of boilerplate in the actual trait method implementation."] # [doc = " It takes an invocation of method as an argument (e.g. `self.poll(cx)`), and redirects it to either"] # [doc = " enum variant held in `self`."] macro_rules ! delegate_call { ($ self : ident .$ method : ident ($ ($ args : ident) ,+)) => { unsafe { match $ self . get_unchecked_mut () { Self :: Left (l) => Pin :: new_unchecked (l) .$ method ($ ($ args) ,+) , Self :: Right (r) => Pin :: new_unchecked (r) .$ method ($ ($ args) ,+) , } } } }
    };
}

delegate_call!()