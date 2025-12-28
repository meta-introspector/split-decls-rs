macro_rules! deps {
    () => {
        Config!();
        Shard!();
        InitGuard!();
        Clear!();
        DefaultConfig!();
    };
}

macro_rules! RefMut {
    () => {
        deps!();
        # [doc = " A guard that allows exclusive mutable access to an object in a pool."] # [doc = ""] # [doc = " While the guard exists, it indicates to the pool that the item the guard"] # [doc = " references is currently being accessed. If the item is removed from the pool"] # [doc = " while a guard exists, the removal will be deferred until the guard is"] # [doc = " dropped. The slot cannot be accessed by other threads while it is accessed"] # [doc = " mutably."] pub struct RefMut < 'a , T , C = DefaultConfig > where T : Clear + Default , C : cfg :: Config , { inner : page :: slot :: InitGuard < T , C > , shard : & 'a Shard < T , C > , key : usize , }
    };
}

RefMut!()