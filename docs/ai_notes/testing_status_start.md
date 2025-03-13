# Test Status at Start of Refactor:
The following is the output of `cargo test` with all the compiler warnings
removed. This should serve as a grounding point to help determine the progress
of the ID refactor.

It is possible that some of the below tests will fail, or even result in
compiler errors as the types of the IDs are changed. When that happens, it
would be better to go in and change the tests so they reflect the new state of
the ID types. However, we must be careful when altering the tests that they are
not trivially passing, so maintaining or improving the logic in the test will
be important.

Output of `cargo test` with compiler warnings removed: Dated commit
--

     Finished `test` profile [unoptimized + debuginfo] target(s) in 0.37s
     Running unittests src/lib.rs (target/debug/deps/syn_parser-e8afed13039baab4)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/mod.rs (target/debug/deps/mod-c3c2ff94b8b33154)

running 36 tests
test integration::full_graph_tests::test_full_graph_generation_and_serialization ... ok
test parser::basic_parsing::test_basic_parsing ... ok
test parser::enums_tests::test_enum_parsing ... ok
test parser::functions_tests::test_where_clause_function_parsing ... ok
test parser::functions_tests::test_async_function_parsing ... ok
test parser::functions_tests::test_attributed_function_parsing ... ok
test parser::functions_tests::test_default_params_function_parsing ... ok
test parser::impls_tests::test_find_impl_by_name ... FAILED
test parser::functions_tests::test_documented_function_parsing ... FAILED
test parser::functions_tests::test_multi_generic_function_parsing ... ok
test parser::functions_tests::test_function_with_params_parsing ... ok
test parser::functions_tests::test_regular_function_parsing ... ok
test parser::impls_tests::test_generic_impl_for_struct ... FAILED
test parser::functions_tests::test_private_function_parsing ... ok
test parser::functions_tests::test_generic_function_parsing ... ok
test parser::functions_tests::test_lifetime_function_parsing ... FAILED
test parser::macros_tests::test_macro_parsing ... ok
test parser::functions_tests::test_unsafe_function_parsing ... ok
test parser::modules_tests::test_module_parsing ... FAILED
test parser::structs_tests::test_struct_with_attributes ... ok
test parser::structs_tests::test_struct_with_docstring ... FAILED
test parser::structs_tests::test_regular_struct_parsing ... ok
test parser::traits_tests::test_trait_parsing ... ok
test serialization::json_tests::test_json_serialization ... ok
test serialization::ron_tests::test_ron_serialization ... ok
test parser::structs_tests::test_unit_struct_parsing ... ok
test parser::structs_tests::test_tuple_struct_parsing ... ok
test parser::structs_tests::test_struct_with_generics ... ok
test parser::impls_tests::test_generic_impl_for_trait ... FAILED
test parser::traits_tests::test_assoc_type_trait_parsing ... ok
test parser::traits_tests::test_generic_trait_parsing ... ok
test parser::traits_tests::test_private_trait_parsing ... ok
test parser::traits_tests::test_default_trait_parsing ... ok
test parser::impls_tests::test_impl_for_struct ... FAILED
test parser::traits_tests::test_regular_trait_parsing ... ok
test parser::impls_tests::test_impl_for_trait ... FAILED

failures:

---- parser::impls_tests::test_find_impl_by_name stdout ----

thread 'parser::impls_tests::test_find_impl_by_name' panicked at tests/parser/impls_tests.rs:99:10:
Impl for SampleTrait not found
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

---- parser::functions_tests::test_documented_function_parsing stdout ----

thread 'parser::functions_tests::test_documented_function_parsing' panicked at tests/parser/functions_tests.rs:88:5:
assertion failed: function.docstring.is_some()

---- parser::impls_tests::test_generic_impl_for_struct stdout ----

thread 'parser::impls_tests::test_generic_impl_for_struct' panicked at tests/parser/impls_tests.rs:146:10:
Impl for GenericStruct not found

---- parser::functions_tests::test_lifetime_function_parsing stdout ----

thread 'parser::functions_tests::test_lifetime_function_parsing' panicked at tests/parser/functions_tests.rs:130:193:
Reference type with lifetime 'a' not found

---- parser::modules_tests::test_module_parsing stdout ----
Module: root, Visibility: Inherited
Submodules: [1]
Items: []
Module: inner_module, Visibility: Inherited
Submodules: []
Items: []

thread 'parser::modules_tests::test_module_parsing' panicked at tests/parser/modules_tests.rs:28:9:
assertion `left == right` failed: Inner module should have exactly one item, but it has 0
  left: 0
 right: 1

---- parser::structs_tests::test_struct_with_docstring stdout ----

thread 'parser::structs_tests::test_struct_with_docstring' panicked at tests/parser/structs_tests.rs:70:5:
assertion failed: documented_struct.docstring.is_some()

---- parser::impls_tests::test_generic_impl_for_trait stdout ----

thread 'parser::impls_tests::test_generic_impl_for_trait' panicked at tests/parser/impls_tests.rs:199:10:
Impl of GenericTrait for GenericStruct not found

---- parser::impls_tests::test_impl_for_struct stdout ----

thread 'parser::impls_tests::test_impl_for_struct' panicked at tests/parser/impls_tests.rs:29:10:
Impl for SampleStruct not found

---- parser::impls_tests::test_impl_for_trait stdout ----

thread 'parser::impls_tests::test_impl_for_trait' panicked at tests/parser/impls_tests.rs:67:10:
Impl of SampleTrait for SampleStruct not found


failures:
    parser::functions_tests::test_documented_function_parsing
    parser::functions_tests::test_lifetime_function_parsing
    parser::impls_tests::test_find_impl_by_name
    parser::impls_tests::test_generic_impl_for_struct
    parser::impls_tests::test_generic_impl_for_trait
    parser::impls_tests::test_impl_for_struct
    parser::impls_tests::test_impl_for_trait
    parser::modules_tests::test_module_parsing
    parser::structs_tests::test_struct_with_docstring

test result: FAILED. 27 passed; 9 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
