macro_rules! deps {
    () => {
        Visitable!();
        RigidTy!();
        Visitor!();
    };
}

macro_rules! impl_470 {
    () => {
        deps!();
        impl Visitable for RigidTy { fn super_visit < V : Visitor > (& self , visitor : & mut V) -> ControlFlow < V :: Break > { match self { RigidTy :: Bool | RigidTy :: Char | RigidTy :: Int (_) | RigidTy :: Uint (_) | RigidTy :: Float (_) | RigidTy :: Never | RigidTy :: Foreign (_) | RigidTy :: Str => ControlFlow :: Continue (()) , RigidTy :: Array (t , c) => { t . visit (visitor) ? ; c . visit (visitor) } RigidTy :: Pat (t , _p) => t . visit (visitor) , RigidTy :: Slice (inner) => inner . visit (visitor) , RigidTy :: RawPtr (ty , _) => ty . visit (visitor) , RigidTy :: Ref (reg , ty , _) => { reg . visit (visitor) ? ; ty . visit (visitor) } RigidTy :: Adt (_ , args) | RigidTy :: Closure (_ , args) | RigidTy :: Coroutine (_ , args) | RigidTy :: CoroutineWitness (_ , args) | RigidTy :: CoroutineClosure (_ , args) | RigidTy :: FnDef (_ , args) => args . visit (visitor) , RigidTy :: FnPtr (sig) => sig . visit (visitor) , RigidTy :: Dynamic (pred , r , _) => { pred . visit (visitor) ? ; r . visit (visitor) } RigidTy :: Tuple (fields) => fields . visit (visitor) , } } }
    };
}

impl_470!();