macro_rules! Ty {
    () => {
        # [derive (Copy , Clone , Eq , PartialEq , Hash , Serialize)] pub struct Ty (usize) ;
    };
}

Ty!();