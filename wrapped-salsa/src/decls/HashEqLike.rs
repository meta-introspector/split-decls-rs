macro_rules! HashEqLike {
    () => {
        # [doc = " A trait for types that hash and compare like `O`."] pub trait HashEqLike < O > { fn hash < H : Hasher > (& self , h : & mut H) ; fn eq (& self , data : & O) -> bool ; }
    };
}

HashEqLike!();