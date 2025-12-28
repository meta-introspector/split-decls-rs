macro_rules! deps {
    () => {
        ArrayVec!();
        Array!();
    };
}

macro_rules! impl_7 {
    () => {
        deps!();
        impl < A > Clone for ArrayVec < A > where A : Array + Clone , A :: Item : Clone , { # [inline] fn clone (& self) -> Self { Self { data : self . data . clone () , len : self . len } } # [inline] fn clone_from (& mut self , o : & Self) { let iter = self . data . as_slice_mut () . iter_mut () . zip (o . data . as_slice ()) . take (self . len . max (o . len) as usize) ; for (dst , src) in iter { dst . clone_from (src) } if let Some (to_drop) = self . data . as_slice_mut () . get_mut ((o . len as usize) .. (self . len as usize)) { to_drop . iter_mut () . for_each (| x | drop (core :: mem :: take (x))) ; } self . len = o . len ; } }
    };
}

impl_7!()