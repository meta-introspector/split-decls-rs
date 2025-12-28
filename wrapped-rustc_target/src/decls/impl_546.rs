macro_rules! deps {
    () => {
        TargetTuple!();
    };
}

macro_rules! impl_546 {
    () => {
        deps!();
        impl PartialEq for TargetTuple { fn eq (& self , other : & Self) -> bool { match (self , other) { (Self :: TargetTuple (l0) , Self :: TargetTuple (r0)) => l0 == r0 , (Self :: TargetJson { path_for_rustdoc : _ , tuple : l_tuple , contents : l_contents } , Self :: TargetJson { path_for_rustdoc : _ , tuple : r_tuple , contents : r_contents } ,) => l_tuple == r_tuple && l_contents == r_contents , _ => false , } } }
    };
}

impl_546!();