macro_rules! deps {
    () => {
        Config!();
        Shared!();
        Local!();
    };
}

macro_rules! Shard {
    () => {
        deps!();
        pub (crate) struct Shard < T , C : cfg :: Config > { # [doc = " The shard's parent thread ID."] pub (crate) tid : usize , # [doc = " The local free list for each page."] # [doc = ""] # [doc = " These are only ever accessed from this shard's thread, so they are"] # [doc = " stored separately from the shared state for the page that can be"] # [doc = " accessed concurrently, to minimize false sharing."] local : Box < [page :: Local] > , # [doc = " The shared state for each page in this shard."] # [doc = ""] # [doc = " This consists of the page's metadata (size, previous size), remote free"] # [doc = " list, and a pointer to the actual array backing that page."] shared : Box < [page :: Shared < T , C >] > , }
    };
}

Shard!();