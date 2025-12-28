macro_rules! deps {
    () => {
        Field!();
        FieldSet!();
        Iter!();
    };
}

macro_rules! impl_180 {
    () => {
        deps!();
        impl Iterator for Iter { type Item = Field ; # [inline] fn next (& mut self) -> Option < Field > { let i = self . idxs . next () ? ; Some (Field { i , fields : FieldSet { names : self . fields . names , callsite : self . fields . callsite () , } , }) } }
    };
}

impl_180!()