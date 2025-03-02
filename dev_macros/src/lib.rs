// todo: FOR dev_utils: dev_macros/some(mod)
// todo:     - some custom proc-macro to gen:
// todo:         - new() fn should have default values for all fields
// todo:         - new(*args) fn should have custom values for all fields



// In the macro_crate/src/lib.rs

// extern crate proc_macro;
// use proc_macro::TokenStream;
// use quote::quote;
// use syn::*;

// #[proc_macro]
// pub fn extract_app_data(input: TokenStream) -> TokenStream {
//     let args = parse_macro_input!(input as syn::ExprArray);

//     let output = quote! {
//         {
//             let mut data = std::collections::HashMap::new();
//             let content = std::fs::read_to_string("Cargo.toml").unwrap();
//             for line in content.lines() {
//                 #(
//                     if line.contains(#args) {
//                         let parts: Vec<&str> = line.split("=").map(|part| part.trim().trim_matches('"')).collect();
//                         if parts.len() == 2 {
//                             data.insert(#args, parts[1]);
//                         }
//                     }
//                 )*
//             }
//             data
//         }
//     };

//     output.into()
// }












// macro_rules! extract_app_data {
//     ($path:expr, $($key:expr),*) => {{
//         use std::collections::HashMap;
//         use std::fs;

//         let mut data = HashMap::new();
//         let content = fs::read_to_string($path).expect("Failed to read file");

//         for line in content.lines() {
//             $(
//                 if line.contains($key) {
//                     let parts: Vec<&str> = line.split('=').map(|part| part.trim_matches('"').trim()).collect();
//                     if parts.len() > 1 {
//                         data.insert($key, parts[1]);
//                     }
//                 }
//             )*
//         }
//         data
//     }};
// }











































// pub fn add(left: usize, right: usize) -> usize {
//     left + right
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
