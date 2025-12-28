macro_rules! deps {
    () => {
        DefaultCallsite!();
        Metadata!();
        Interest!();
    };
}

macro_rules! impl_42 {
    () => {
        deps!();
        impl DefaultCallsite { const UNREGISTERED : u8 = 0 ; const REGISTERING : u8 = 1 ; const REGISTERED : u8 = 2 ; const INTEREST_NEVER : u8 = 0 ; const INTEREST_SOMETIMES : u8 = 1 ; const INTEREST_ALWAYS : u8 = 2 ; # [doc = " Returns a new `DefaultCallsite` with the specified `Metadata`."] pub const fn new (meta : & 'static Metadata < 'static >) -> Self { Self { interest : AtomicU8 :: new (0xFF) , meta , next : AtomicPtr :: new (ptr :: null_mut ()) , registration : AtomicU8 :: new (Self :: UNREGISTERED) , } } # [doc = " Registers this callsite with the global callsite registry."] # [doc = ""] # [doc = " If the callsite is already registered, this does nothing. When using"] # [doc = " [`DefaultCallsite`], this method should be preferred over"] # [doc = " [`tracing_core::callsite::register`], as it ensures that the callsite is"] # [doc = " only registered a single time."] # [doc = ""] # [doc = " Other callsite implementations will generally ensure that"] # [doc = " callsites are not re-registered through another mechanism."] # [doc = ""] # [doc = " See the [documentation on callsite registration][reg-docs] for details"] # [doc = " on the global callsite registry."] # [doc = ""] # [doc = " [`tracing_core::callsite::register`]: crate::callsite::register"] # [doc = " [reg-docs]: crate::callsite#registering-callsites"] # [inline (never)] # [cold] pub fn register (& 'static self) -> Interest { match self . registration . compare_exchange (Self :: UNREGISTERED , Self :: REGISTERING , Ordering :: AcqRel , Ordering :: Acquire ,) { Ok (_) => { CALLSITES . push_default (self) ; rebuild_callsite_interest (self , & DISPATCHERS . rebuilder ()) ; self . registration . store (Self :: REGISTERED , Ordering :: Release) ; } Err (Self :: REGISTERED) => { } Err (_state) => { debug_assert_eq ! (_state , Self :: REGISTERING , "weird callsite registration state") ; return Interest :: sometimes () ; } } match self . interest . load (Ordering :: Relaxed) { Self :: INTEREST_NEVER => Interest :: never () , Self :: INTEREST_ALWAYS => Interest :: always () , _ => Interest :: sometimes () , } } # [doc = " Returns the callsite's cached `Interest`, or registers it for the"] # [doc = " first time if it has not yet been registered."] # [inline] pub fn interest (& 'static self) -> Interest { match self . interest . load (Ordering :: Relaxed) { Self :: INTEREST_NEVER => Interest :: never () , Self :: INTEREST_SOMETIMES => Interest :: sometimes () , Self :: INTEREST_ALWAYS => Interest :: always () , _ => self . register () , } } }
    };
}

impl_42!()