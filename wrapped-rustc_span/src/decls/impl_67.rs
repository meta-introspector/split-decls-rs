macro_rules! deps {
    () => {
        ExpnData!();
        Span!();
        ExpnId!();
        ExpnKind!();
        Symbol!();
        DefId!();
        Edition!();
        HashStableContext!();
    };
}

macro_rules! impl_67 {
    () => {
        deps!();
        impl ExpnData { pub fn new (kind : ExpnKind , parent : ExpnId , call_site : Span , def_site : Span , allow_internal_unstable : Option < Arc < [Symbol] > > , edition : Edition , macro_def_id : Option < DefId > , parent_module : Option < DefId > , allow_internal_unsafe : bool , local_inner_macros : bool , collapse_debuginfo : bool , hide_backtrace : bool ,) -> ExpnData { ExpnData { kind , parent , call_site , def_site , allow_internal_unstable , edition , macro_def_id , parent_module , disambiguator : 0 , allow_internal_unsafe , local_inner_macros , collapse_debuginfo , hide_backtrace , } } # [doc = " Constructs expansion data with default properties."] pub fn default (kind : ExpnKind , call_site : Span , edition : Edition , macro_def_id : Option < DefId > , parent_module : Option < DefId > ,) -> ExpnData { ExpnData { kind , parent : ExpnId :: root () , call_site , def_site : DUMMY_SP , allow_internal_unstable : None , edition , macro_def_id , parent_module , disambiguator : 0 , allow_internal_unsafe : false , local_inner_macros : false , collapse_debuginfo : false , hide_backtrace : false , } } pub fn allow_unstable (kind : ExpnKind , call_site : Span , edition : Edition , allow_internal_unstable : Arc < [Symbol] > , macro_def_id : Option < DefId > , parent_module : Option < DefId > ,) -> ExpnData { ExpnData { allow_internal_unstable : Some (allow_internal_unstable) , .. ExpnData :: default (kind , call_site , edition , macro_def_id , parent_module) } } # [inline] pub fn is_root (& self) -> bool { matches ! (self . kind , ExpnKind :: Root) } # [inline] fn hash_expn (& self , ctx : & mut impl HashStableContext) -> Hash64 { let mut hasher = StableHasher :: new () ; self . hash_stable (ctx , & mut hasher) ; hasher . finish () } }
    };
}

impl_67!();