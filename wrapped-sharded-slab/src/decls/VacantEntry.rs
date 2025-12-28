macro_rules! deps {
    () => {
        Slab!();
        DefaultConfig!();
        Config!();
        InitGuard!();
    };
}

macro_rules! VacantEntry {
    () => {
        deps!();
        # [doc = " A handle to a vacant entry in a [`Slab`]."] # [doc = ""] # [doc = " `VacantEntry` allows constructing values with the key that they will be"] # [doc = " assigned to."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " # use sharded_slab::Slab;"] # [doc = " let mut slab = Slab::new();"] # [doc = ""] # [doc = " let hello = {"] # [doc = "     let entry = slab.vacant_entry().unwrap();"] # [doc = "     let key = entry.key();"] # [doc = ""] # [doc = "     entry.insert((key, \"hello\"));"] # [doc = "     key"] # [doc = " };"] # [doc = ""] # [doc = " assert_eq!(hello, slab.get(hello).unwrap().0);"] # [doc = " assert_eq!(\"hello\", slab.get(hello).unwrap().1);"] # [doc = " ```"] # [derive (Debug)] pub struct VacantEntry < 'a , T , C : cfg :: Config = DefaultConfig > { inner : page :: slot :: InitGuard < Option < T > , C > , key : usize , _lt : PhantomData < & 'a () > , }
    };
}

VacantEntry!();