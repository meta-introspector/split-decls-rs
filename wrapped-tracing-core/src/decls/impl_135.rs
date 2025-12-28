macro_rules! deps {
    () => {
        Field!();
        Visit!();
        Value!();
    };
}

macro_rules! impl_135 {
    () => {
        deps!();
        # [cfg (feature = "std")] # [cfg_attr (docsrs , doc (cfg (feature = "std")))] impl Value for dyn std :: error :: Error + Send + 'static { fn record (& self , key : & Field , visitor : & mut dyn Visit) { (self as & dyn std :: error :: Error) . record (key , visitor) } }
    };
}

impl_135!();