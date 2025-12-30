// Generated macro for IterableDataProviderCached (trait)
macro_rules! DepcrateIterableDataProviderCached {
() => {
// Module: crate
// Provides: {"IterableDataProviderCached"}
// Dependencies: {}
trait IterableDataProviderCached < M : DataMarker > : DataProvider < M > { fn iter_ids_cached (& self) -> Result < HashSet < DataIdentifierCow < 'static > > , DataError > ; }
};
}
