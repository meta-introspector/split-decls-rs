// Generated macro for impl_47 (impl)
macro_rules! Depcrate_aggregateimpl_47 {
() => {
// Module: crate::aggregate
// Provides: {"impl_47"}
// Dependencies: {}
impl < T : Copy + Ord , S : Clone > Extrema < T , S > { pub fn new (limit : usize) -> Self { Extrema { limit , smallest : BTreeMap :: new () , largest : BTreeMap :: new () , } } pub fn add (& mut self , value : T , source : & S) { self . add_range (value ..= value , source) } pub fn add_range (& mut self , range : std :: ops :: RangeInclusive < T > , source : & S) { enum Which { Smallest , Largest , } for which in & [Which :: Smallest , Which :: Largest] { let (map , & value) = match which { Which :: Smallest => (& mut self . smallest , range . start ()) , Which :: Largest => (& mut self . largest , range . end ()) , } ; if map . len () < self . limit { map . entry (value) . or_default () . add (source) ; } else { let least_extreme = match which { Which :: Smallest => map . keys () . rev () . next () . copied () . unwrap () , Which :: Largest => map . keys () . next () . copied () . unwrap () , } ; let less_extreme = match which { Which :: Smallest => value > least_extreme , Which :: Largest => value < least_extreme , } ; if ! less_extreme { map . entry (value) . or_default () . add (source) ; if map . len () > self . limit { map . remove (& least_extreme) ; } assert_eq ! (map . len () , self . limit) ; } } } } }
};
}
