// https://github.com/bincode-org/virtue/blob/trunk/src/lib.rs#L97-L106
// pub(crate) fn to_token_stream(
//     s: &str,
// ) -> proc_macro2::TokenStream {
//     use std::str::FromStr;

//     proc_macro2::TokenStream::from_str(s)
//         .unwrap_or_else(|e| panic!("Could not parse code: {:?}\n{:?}", s, e))
// }
