macro_rules! deps {
    () => {
        VersionInfo!();
    };
}

macro_rules! impl_4 {
    () => {
        deps!();
        impl std :: fmt :: Debug for VersionInfo { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { write ! (f , "VersionInfo {{ crate_name: \"{}\", major: {}, minor: {}, patch: {}" , self . crate_name , self . major , self . minor , self . patch ,) ? ; if let Some (ref commit_hash) = self . commit_hash { write ! (f , ", commit_hash: \"{}\"" , commit_hash . trim () ,) ? ; } if let Some (ref commit_date) = self . commit_date { write ! (f , ", commit_date: \"{}\"" , commit_date . trim ()) ? ; } if let Some (ref host_compiler) = self . host_compiler { write ! (f , ", host_compiler: \"{}\"" , host_compiler . trim ()) ? ; } write ! (f , " }}") ? ; Ok (()) } }
    };
}

impl_4!()