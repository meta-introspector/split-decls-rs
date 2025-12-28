macro_rules! deps {
    () => {
        ArgAbi!();
        ABI!();
        Uniform!();
    };
}

macro_rules! is_homogeneous_aggregate {
    () => {
        deps!();
        fn is_homogeneous_aggregate < 'a , Ty , C > (cx : & C , arg : & mut ArgAbi < 'a , Ty > , abi : ABI ,) -> Option < Uniform > where Ty : TyAbiInterface < 'a , C > + Copy , C : HasDataLayout , { arg . layout . homogeneous_aggregate (cx) . ok () . and_then (| ha | ha . unit ()) . and_then (| unit | { if ((abi == ELFv1 || abi == AIX) && arg . layout . size > unit . size) || arg . layout . size > unit . size . checked_mul (8 , cx) . unwrap () { return None ; } let valid_unit = match unit . kind { RegKind :: Integer => false , RegKind :: Float => true , RegKind :: Vector => arg . layout . size . bits () == 128 , } ; valid_unit . then_some (Uniform :: consecutive (unit , arg . layout . size)) }) }
    };
}

is_homogeneous_aggregate!();