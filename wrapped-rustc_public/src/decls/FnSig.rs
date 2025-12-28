macro_rules! deps {
    () => {
        Ty!();
        Abi!();
    };
}

macro_rules! FnSig {
    () => {
        deps!();
        # [derive (Clone , Debug , Eq , PartialEq , Serialize)] pub struct FnSig { pub inputs_and_output : Vec < Ty > , pub c_variadic : bool , pub safety : Safety , pub abi : Abi , }
    };
}

FnSig!();