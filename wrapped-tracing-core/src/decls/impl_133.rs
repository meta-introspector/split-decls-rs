macro_rules! deps {
    () => {
        Field!();
        Visit!();
        Value!();
    };
}

macro_rules! impl_133 {
    () => {
        deps!();
        # [cfg (feature = "std")] # [cfg_attr (docsrs , doc (cfg (feature = "std")))] impl Value for dyn std :: error :: Error + 'static { fn record (& self , key : & Field , visitor : & mut dyn Visit) { visitor . record_error (key , self) } }
    };
}

impl_133!();