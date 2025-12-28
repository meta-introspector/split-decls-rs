macro_rules! deps {
    () => {
        Slab!();
    };
}

macro_rules! Entry {
    () => {
        deps!();
        # [doc = " A handle that allows access to an occupied entry in a [`Slab`]."] # [doc = ""] # [doc = " While the guard exists, it indicates to the slab that the item the guard"] # [doc = " references is currently being accessed. If the item is removed from the slab"] # [doc = " while a guard exists, the removal will be deferred until all guards are"] # [doc = " dropped."] pub struct Entry < 'a , T , C : cfg :: Config = DefaultConfig > { inner : page :: slot :: Guard < Option < T > , C > , value : ptr :: NonNull < T > , shard : & 'a Shard < Option < T > , C > , key : usize , }
    };
}

Entry!()