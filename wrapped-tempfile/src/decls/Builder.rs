macro_rules! Builder {
    () => {
        # [doc = " Create a new temporary file or directory with custom options."] # [derive (Debug , Clone , Eq , PartialEq)] pub struct Builder < 'a , 'b > { random_len : usize , prefix : & 'a OsStr , suffix : & 'b OsStr , append : bool , permissions : Option < Permissions > , disable_cleanup : bool , }
    };
}

Builder!();