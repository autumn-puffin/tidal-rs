use attributes::{
  fields::*,
  requests::{Method, RequestFunction},
};
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::ToTokens;
use syn::ItemImpl;

mod attributes;

#[proc_macro_attribute]
pub fn get(args: TokenStream, input: TokenStream) -> TokenStream {
  let request = RequestFunction::new_from_attrib(Method::Get, args, input);
  let output: TokenStream2 = request.generate();
  output.into()
}
#[proc_macro_attribute]
pub fn post(args: TokenStream, input: TokenStream) -> TokenStream {
  let request = RequestFunction::new_from_attrib(Method::Post, args, input);
  let output: TokenStream2 = request.generate();
  output.into()
}
#[proc_macro_attribute]
pub fn put(args: TokenStream, input: TokenStream) -> TokenStream {
  let request = RequestFunction::new_from_attrib(Method::Put, args, input);
  let output: TokenStream2 = request.generate();
  output.into()
}
#[proc_macro_attribute]
pub fn delete(args: TokenStream, input: TokenStream) -> TokenStream {
  let request = RequestFunction::new_from_attrib(Method::Delete, args, input);
  let output: TokenStream2 = request.generate();
  output.into()
}

fn field_attribute<T: Field>(args: TokenStream, input: TokenStream) -> TokenStream {
  if let Ok(mut impl_block) = syn::parse::<ItemImpl>(input.clone()) {
    let field: T = syn::parse(args).unwrap();
    field.add_to_impl_members(&mut impl_block);
    impl_block.into_token_stream().into()
  } else {
    input
  }
}

#[proc_macro_attribute]
pub fn client(args: TokenStream, input: TokenStream) -> TokenStream {
  field_attribute::<Client>(args, input)
}
#[proc_macro_attribute]
pub fn base_url(args: TokenStream, input: TokenStream) -> TokenStream {
  field_attribute::<BaseUrl>(args, input)
}
#[proc_macro_attribute]
pub fn headers(args: TokenStream, input: TokenStream) -> TokenStream {
  field_attribute::<Headers>(args, input)
}
#[proc_macro_attribute]
pub fn body(args: TokenStream, input: TokenStream) -> TokenStream {
  field_attribute::<Body>(args, input)
}
#[proc_macro_attribute]
pub fn query(args: TokenStream, input: TokenStream) -> TokenStream {
  field_attribute::<Query>(args, input)
}
#[proc_macro_attribute]
pub fn shared_query(args: TokenStream, input: TokenStream) -> TokenStream {
  field_attribute::<SharedQuery>(args, input)
}
#[proc_macro_attribute]
pub fn basic_auth(args: TokenStream, input: TokenStream) -> TokenStream {
  field_attribute::<BasicAuth>(args, input)
}
#[proc_macro_attribute]
pub fn bearer_auth(args: TokenStream, input: TokenStream) -> TokenStream {
  field_attribute::<BearerAuth>(args, input)
}
#[proc_macro_attribute]
pub fn response_handler(args: TokenStream, input: TokenStream) -> TokenStream {
  field_attribute::<ResponseHandler>(args, input)
}

#[proc_macro_attribute]
pub fn body_raw(_args: TokenStream, input: TokenStream) -> TokenStream {
  input
}
#[proc_macro_attribute]
pub fn body_json(_args: TokenStream, input: TokenStream) -> TokenStream {
  input
}
#[proc_macro_attribute]
pub fn body_form_url_encoded(_args: TokenStream, input: TokenStream) -> TokenStream {
  input
}
#[proc_macro_attribute]
pub fn body_multipart(_args: TokenStream, input: TokenStream) -> TokenStream {
  input
}
