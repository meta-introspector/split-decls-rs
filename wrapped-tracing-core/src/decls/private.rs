macro_rules! deps {
    () => {
        ValueSet!();
        Field!();
        Value!();
    };
}

macro_rules! private {
    () => {
        deps!();
        mod private { use super :: * ; # [doc = " Restrictions on `ValueSet` lengths were removed in #2508 but this type remains for backwards compatibility."] pub trait ValidLen < 'a > : Borrow < [(& 'a Field , Option < & 'a (dyn Value + 'a) >)] > { } impl < 'a , const N : usize > ValidLen < 'a > for [(& 'a Field , Option < & 'a (dyn Value + 'a) >) ; N] { } }
    };
}

private!()