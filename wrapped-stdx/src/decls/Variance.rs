macro_rules! Variance {
    () => {
        # [doc = " A marker trait for phantom variance types."] pub trait Variance : sealed :: Sealed + Default { }
    };
}

Variance!()