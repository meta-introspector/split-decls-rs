macro_rules! Method {
    () => {
        # [repr (i32)] # [derive (Debug , Clone , Copy , PartialEq , Eq , Hash , Default)] # [cfg_attr (feature = "__internal-fuzz" , derive (arbitrary :: Arbitrary))] pub enum Method { # [default] Deflated = 8 , }
    };
}

Method!();