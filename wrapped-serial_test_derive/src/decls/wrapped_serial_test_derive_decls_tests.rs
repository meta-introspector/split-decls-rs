use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[cfg(test)]
mod tests {
    use super::{fs_serial_core, local_serial_core};
    use proc_macro2::TokenStream;
    use quote::quote;
    use std::iter::FromIterator;
    fn init() {
        let _ = env_logger::builder().is_test(false).try_init();
    }
    fn unparse(input: TokenStream) -> String {
        let item = syn::parse2(input).unwrap();
        let file = syn::File {
            attrs: vec![],
            items: vec![item],
            shebang: None,
        };
        prettyplease::unparse(&file)
    }
    fn compare_streams(first: TokenStream, second: TokenStream) {
        let f = unparse(first);
        assert_eq!(f, unparse(second));
    }
    #[test]
    fn test_serial() {
        init();
        let attrs = proc_macro2::TokenStream::new();
        let input = quote! {
            #[test] fn foo() {}
        };
        let stream = local_serial_core(attrs.into(), input);
        let compare = quote! {
            #[test] fn foo() { serial_test::local_serial_core(vec![""],
            ::std::option::Option::None, || {}); }
        };
        compare_streams(compare, stream);
    }
    #[test]
    fn test_serial_with_pub() {
        init();
        let attrs = proc_macro2::TokenStream::new();
        let input = quote! {
            #[test] pub fn foo() {}
        };
        let stream = local_serial_core(attrs.into(), input);
        let compare = quote! {
            #[test] pub fn foo() { serial_test::local_serial_core(vec![""],
            ::std::option::Option::None, || {}); }
        };
        compare_streams(compare, stream);
    }
    #[test]
    fn test_other_attributes() {
        init();
        let attrs = proc_macro2::TokenStream::new();
        let input = quote! {
            #[test] #[ignore] #[should_panic(expected = "Testing panic")]
            #[something_else] fn foo() {}
        };
        let stream = local_serial_core(attrs.into(), input);
        let compare = quote! {
            #[test] #[ignore] #[should_panic(expected = "Testing panic")]
            #[something_else] fn foo() { serial_test::local_serial_core(vec![""],
            ::std::option::Option::None, || {}); }
        };
        compare_streams(compare, stream);
    }
    #[test]
    #[cfg(feature = "async")]
    fn test_serial_async() {
        init();
        let attrs = proc_macro2::TokenStream::new();
        let input = quote! {
            async fn foo() {}
        };
        let stream = local_serial_core(attrs.into(), input);
        let compare = quote! {
            async fn foo() { async fn _foo_internal() {}
            serial_test::local_async_serial_core(vec![""], ::std::option::Option::None,
            _foo_internal()). await; }
        };
        assert_eq!(format!("{}", compare), format!("{}", stream));
    }
    #[test]
    #[cfg(feature = "async")]
    fn test_serial_async_return() {
        init();
        let attrs = proc_macro2::TokenStream::new();
        let input = quote! {
            async fn foo() -> Result < (), () > { Ok(()) }
        };
        let stream = local_serial_core(attrs.into(), input);
        let compare = quote! {
            async fn foo() -> Result < (), () > { async fn _foo_internal() -> Result <
            (), () > { Ok(()) }
            serial_test::local_async_serial_core_with_return(vec![""],
            ::std::option::Option::None, _foo_internal()). await }
        };
        assert_eq!(format!("{}", compare), format!("{}", stream));
    }
    #[test]
    fn test_file_serial() {
        init();
        let attrs: Vec<_> = quote! {
            foo
        }
        .into_iter()
        .collect();
        let input = quote! {
            #[test] fn foo() {}
        };
        let stream = fs_serial_core(
            proc_macro2::TokenStream::from_iter(attrs.into_iter()),
            input,
        );
        let compare = quote! {
            #[test] fn foo() { serial_test::fs_serial_core(vec!["foo"],
            ::std::option::Option::None, || {}); }
        };
        compare_streams(compare, stream);
    }
    #[test]
    fn test_file_serial_no_args() {
        init();
        let attrs = proc_macro2::TokenStream::new();
        let input = quote! {
            #[test] fn foo() {}
        };
        let stream = fs_serial_core(
            proc_macro2::TokenStream::from_iter(attrs.into_iter()),
            input,
        );
        let compare = quote! {
            #[test] fn foo() { serial_test::fs_serial_core(vec![""],
            ::std::option::Option::None, || {}); }
        };
        compare_streams(compare, stream);
    }
    #[test]
    fn test_file_serial_with_path() {
        init();
        let attrs: Vec<_> = quote! {
            foo, path => "bar_path"
        }
        .into_iter()
        .collect();
        let input = quote! {
            #[test] fn foo() {}
        };
        let stream = fs_serial_core(
            proc_macro2::TokenStream::from_iter(attrs.into_iter()),
            input,
        );
        let compare = quote! {
            #[test] fn foo() { serial_test::fs_serial_core(vec!["foo"],
            ::std::option::Option::Some("bar_path"), || {}); }
        };
        compare_streams(compare, stream);
    }
    #[test]
    fn test_single_attr() {
        init();
        let attrs: Vec<_> = quote! {
            one
        }
        .into_iter()
        .collect();
        let input = quote! {
            #[test] fn single() {}
        };
        let stream = local_serial_core(
            proc_macro2::TokenStream::from_iter(attrs.into_iter()),
            input,
        );
        let compare = quote! {
            #[test] fn single() { serial_test::local_serial_core(vec!["one"],
            ::std::option::Option::None, || {}); }
        };
        compare_streams(compare, stream);
    }
    #[test]
    fn test_multiple_attr() {
        init();
        let attrs: Vec<_> = quote! {
            two, one
        }
        .into_iter()
        .collect();
        let input = quote! {
            #[test] fn multiple() {}
        };
        let stream = local_serial_core(
            proc_macro2::TokenStream::from_iter(attrs.into_iter()),
            input,
        );
        let compare = quote! {
            #[test] fn multiple() { serial_test::local_serial_core(vec!["one", "two"],
            ::std::option::Option::None, || {}); }
        };
        compare_streams(compare, stream);
    }
    #[test]
    fn test_mod() {
        init();
        let attrs = proc_macro2::TokenStream::new();
        let input = quote! {
            #[cfg(test)] #[serial] mod serial_attr_tests { pub fn foo() {
            println!("Nothing"); } #[test] fn bar() {} }
        };
        let stream = local_serial_core(
            proc_macro2::TokenStream::from_iter(attrs.into_iter()),
            input,
        );
        let compare = quote! {
            #[cfg(test)] mod serial_attr_tests { pub fn foo() { println!("Nothing"); }
            #[test] fn bar() { serial_test::local_serial_core(vec![""],
            ::std::option::Option::None, || {}); } }
        };
        compare_streams(compare, stream);
    }
    #[test]
    fn test_later_test_mod() {
        init();
        let attrs = proc_macro2::TokenStream::new();
        let input = quote! {
            #[cfg(test)] #[serial] mod serial_attr_tests { pub fn foo() {
            println!("Nothing"); } #[demo_library::test] fn bar() {} }
        };
        let stream = local_serial_core(
            proc_macro2::TokenStream::from_iter(attrs.into_iter()),
            input,
        );
        let compare = quote! {
            #[cfg(test)] mod serial_attr_tests { pub fn foo() { println!("Nothing"); }
            #[demo_library::test] fn bar() { serial_test::local_serial_core(vec![""],
            ::std::option::Option::None, || {}); } }
        };
        compare_streams(compare, stream);
    }
    #[test]
    #[cfg(feature = "async")]
    fn test_mod_with_async() {
        init();
        let attrs = proc_macro2::TokenStream::new();
        let input = quote! {
            #[cfg(test)] #[serial] mod serial_attr_tests { #[demo_library::test] async fn
            foo() -> Result < (), () > { Ok(()) } #[demo_library::test] #[ignore = "bla"]
            async fn bar() -> Result < (), () > { Ok(()) } }
        };
        let stream = local_serial_core(
            proc_macro2::TokenStream::from_iter(attrs.into_iter()),
            input,
        );
        let compare = quote! {
            #[cfg(test)] mod serial_attr_tests { #[demo_library::test] async fn foo() ->
            Result < (), () > { async fn _foo_internal() -> Result < (), () > { Ok(()) }
            serial_test::local_async_serial_core_with_return(vec![""],
            ::std::option::Option::None, _foo_internal()). await } #[demo_library::test]
            #[ignore = "bla"] async fn bar() -> Result < (), () > { async fn
            _bar_internal() -> Result < (), () > { Ok(()) }
            serial_test::local_async_serial_core_with_return(vec![""],
            ::std::option::Option::None, _bar_internal()). await } }
        };
        compare_streams(compare, stream);
    }
    #[test]
    fn test_nested_return() {
        init();
        let attrs = proc_macro2::TokenStream::new();
        let input = quote! {
            #[test] fn test() -> Result < Result < (), () >, () > { Ok(Ok(())) }
        };
        let stream = local_serial_core(
            proc_macro2::TokenStream::from_iter(attrs.into_iter()),
            input,
        );
        let compare = quote! {
            #[test] fn test() -> Result < Result < (), () >, () > {
            serial_test::local_serial_core_with_return(vec![""],
            ::std::option::Option::None, || { Ok(Ok(())) }) }
        };
        compare_streams(compare, stream);
    }
    #[test]
    fn test_crate_wrapper() {
        init();
        let attrs: Vec<_> = quote! {
            crate = wrapper::__derive_refs::serial
        }
        .into_iter()
        .collect();
        let input = quote! {
            #[test] fn foo() {}
        };
        let stream = fs_serial_core(
            proc_macro2::TokenStream::from_iter(attrs.into_iter()),
            input,
        );
        let compare = quote! {
            #[test] fn foo() { wrapper::__derive_refs::serial::fs_serial_core(vec![""],
            ::std::option::Option::None, || {}); }
        };
        compare_streams(compare, stream);
    }
    #[test]
    fn test_crate_wrapper_with_path() {
        init();
        let attrs: Vec<_> = quote! {
            crate = wrapper::__derive_refs::serial, path => "/tmp/bar"
        }
        .into_iter()
        .collect();
        let input = quote! {
            #[test] fn foo() {}
        };
        let stream = fs_serial_core(
            proc_macro2::TokenStream::from_iter(attrs.into_iter()),
            input,
        );
        let compare = quote! {
            #[test] fn foo() { wrapper::__derive_refs::serial::fs_serial_core(vec![""],
            ::std::option::Option::Some("/tmp/bar"), || {}); }
        };
        compare_streams(compare, stream);
    }
    #[test]
    fn test_crate_wrapper_with_path_and_key() {
        init();
        let attrs: Vec<_> = quote! {
            key1, key2, path => "/tmp/bar", crate = wrapper::__derive_refs::serial
        }
        .into_iter()
        .collect();
        let input = quote! {
            #[test] fn foo() {}
        };
        let stream = fs_serial_core(
            proc_macro2::TokenStream::from_iter(attrs.into_iter()),
            input,
        );
        let compare = quote! {
            #[test] fn foo() {
            wrapper::__derive_refs::serial::fs_serial_core(vec!["key1", "key2"],
            ::std::option::Option::Some("/tmp/bar"), || {}); }
        };
        compare_streams(compare, stream);
    }
}
