macro_rules! deps {
    () => {
        Channel!();
    };
}

macro_rules! impl_60 {
    () => {
        deps!();
        impl < T > Channel < T > { # [doc = " Creates a new channel with nothing in it."] pub fn new () -> Self { let storage = Default :: default () ; let me = Self { storage , empty : AtomicU16 :: new (0) , full : AtomicU16 :: new (0) , } ; for i in 1 .. SLOTS + 1 { enqueue (& me . empty , i as u16) ; } me } # [doc = " Inserts a value into the channel."] # [doc = ""] # [doc = " If the value doesn't fit, it is silently dropped. Never blocks."] pub fn send (& self , val : T) { if let Some (empty_idx) = dequeue (& self . empty) { unsafe { * self . storage [empty_idx as usize - 1] . get () = Some (val) } ; enqueue (& self . full , empty_idx) ; } } # [doc = " Takes a value from the channel."] # [doc = ""] # [doc = " Or returns `None` if the channel is empty. Never blocks."] pub fn recv (& self) -> Option < T > { dequeue (& self . full) . map (| idx | { let result = unsafe { & mut * self . storage [idx as usize - 1] . get () } . take () . expect ("Full slot with nothing in it") ; enqueue (& self . empty , idx) ; result }) } }
    };
}

impl_60!();