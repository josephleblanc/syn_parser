// Fix these up later
// #[macro_use]
// extern crate criterion;
// use criterion::Criterion;
// use syn::parse_quote;
// use syn_parser::parser::visitor::state::VisitorState;
// use syn_parser::parser::visitor::type_processing::TypeProcessor;
//
// fn benchmark_type_processing(c: &mut Criterion) {
//     c.bench_function("complex_type_resolution", |b| {
//         b.iter(|| {
//             let mut state = VisitorState::new();
//             let ty: syn::Type = parse_quote! {
//                 HashMap<&'a mut [u32], Box<dyn Iterator<Item=Option<T>>>>
//             };
//             state.get_or_create_type(&ty)
//         })
//     });
// }
//
// criterion_group!(benches, benchmark_type_processing);
// criterion_main!(benches);
