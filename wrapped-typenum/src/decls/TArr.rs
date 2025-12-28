macro_rules! deps {
    () => {
        UInt!();
        Eq!();
        Integer!();
        Ord!();
    };
}

macro_rules! TArr {
    () => {
        deps!();
        # [doc = " `TArr` is a type that acts as an array of types. It is defined similarly to `UInt`, only its"] # [doc = " values can be more than bits, and it is designed to act as an array. So you can only add two if"] # [doc = " they have the same number of elements, for example."] # [doc = ""] # [doc = " This array is only really designed to contain `Integer` types. If you use it with others, you"] # [doc = " may find it lacking functionality."] # [derive (Eq , PartialEq , Ord , PartialOrd , Clone , Copy , Hash , Debug)] # [cfg_attr (feature = "scale_info" , derive (scale_info :: TypeInfo))] pub struct TArr < V , A > { first : V , rest : A , }
    };
}

TArr!()