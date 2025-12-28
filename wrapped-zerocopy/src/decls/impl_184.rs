macro_rules! deps {
    () => {
        ConvertError!();
    };
}

macro_rules! impl_184 {
    () => {
        deps!();
        # [cfg (any (zerocopy_core_error_1_81_0 , feature = "std" , test))] # [cfg_attr (doc_cfg , doc (cfg (all (rust = "1.81.0" , feature = "std"))))] impl < A , S , V > Error for ConvertError < A , S , V > where A : fmt :: Display + fmt :: Debug , S : fmt :: Display + fmt :: Debug , V : fmt :: Display + fmt :: Debug , { }
    };
}

impl_184!()