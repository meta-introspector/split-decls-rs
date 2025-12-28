macro_rules! deps {
    () => {
        Value!();
        Visit!();
        Field!();
    };
}

macro_rules! impl_137 {
    () => {
        deps!();
        # [cfg (feature = "std")] # [cfg_attr (docsrs , doc (cfg (feature = "std")))] impl Value for dyn std :: error :: Error + Sync + 'static { fn record (& self , key : & Field , visitor : & mut dyn Visit) { (self as & dyn std :: error :: Error) . record (key , visitor) } }
    };
}

impl_137!()