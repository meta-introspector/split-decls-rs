macro_rules! deps {
    () => {
        Guard!();
        Config!();
        DefaultConfig!();
        Clear!();
        Shard!();
    };
}

macro_rules! Ref {
    () => {
        deps!();
        # [doc = " A guard that allows access to an object in a pool."] # [doc = ""] # [doc = " While the guard exists, it indicates to the pool that the item the guard references is"] # [doc = " currently being accessed. If the item is removed from the pool while the guard exists, the"] # [doc = " removal will be deferred until all guards are dropped."] pub struct Ref < 'a , T , C = DefaultConfig > where T : Clear + Default , C : cfg :: Config , { inner : page :: slot :: Guard < T , C > , shard : & 'a Shard < T , C > , key : usize , }
    };
}

Ref!()