macro_rules! deps {
    () => {
        SliceMut!();
        Slice!();
    };
}

macro_rules! impl_194 {
    () => {
        deps!();
        impl < 'a , A : PartialEq + 'a , const N : usize > PartialEq < Slice < 'a , A , N > > for SliceMut < 'a , A , N > { # [inline] # [must_use] fn eq (& self , other : & Slice < 'a , A , N >) -> bool { self . len () == other . len () && self . iter () . eq (other . iter ()) } }
    };
}

impl_194!();