macro_rules! deps {
    () => {
        Field!();
        Visit!();
        Value!();
    };
}

macro_rules! impl_160 {
    () => {
        deps!();
        # [cfg (all (tracing_unstable , feature = "valuable"))] # [cfg_attr (docsrs , doc (cfg (all (tracing_unstable , feature = "valuable"))))] impl Value for valuable :: Value < '_ > { fn record (& self , key : & Field , visitor : & mut dyn Visit) { visitor . record_value (key , * self) } }
    };
}

impl_160!()