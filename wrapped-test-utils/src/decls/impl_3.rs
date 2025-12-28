macro_rules! deps {
    () => {
        AssertLinear!();
    };
}

macro_rules! impl_3 {
    () => {
        deps!();
        impl Drop for AssertLinear { fn drop (& mut self) { assert ! (! self . rounds . is_empty ()) ; if self . rounds . iter () . all (| it | ! it . linear) { for round in & self . rounds { eprintln ! ("\n{}" , round . plot) ; } panic ! ("Doesn't look linear!") ; } } }
    };
}

impl_3!()