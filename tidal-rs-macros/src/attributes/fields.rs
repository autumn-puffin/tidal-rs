use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::ToTokens;
use syn::{parse::Parse, Attribute, Expr};

fn add_to_impl_members<T: ToTokens + Parse>(attr: &T, attr_name: &str, impl_block: &mut syn::ItemImpl) {
  let attr = attr.to_token_stream();
  let ident = syn::Ident::new(attr_name, Span::call_site());
  let attr: Attribute = syn::parse_quote! {#[#ident(#attr)]};
  for item in &mut impl_block.items {
    let syn::ImplItem::Fn(func) = item else { continue };
    func.attrs.push(attr.clone());
  }
}

pub trait Field: Parse + ToTokens {
  fn add_to_impl_members(&self, impl_block: &mut syn::ItemImpl);
}

pub struct Client(Expr);
impl Parse for Client {
  fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
    Ok(Self(input.parse()?))
  }
}
impl ToTokens for Client {
  fn to_tokens(&self, tokens: &mut TokenStream2) {
    self.0.to_tokens(tokens);
  }
}
impl Field for Client {
  fn add_to_impl_members(&self, impl_block: &mut syn::ItemImpl) {
    add_to_impl_members(self, "client", impl_block);
  }
}

pub struct BaseUrl(Expr);
impl Parse for BaseUrl {
  fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
    Ok(Self(input.parse()?))
  }
}
impl ToTokens for BaseUrl {
  fn to_tokens(&self, tokens: &mut TokenStream2) {
    self.0.to_tokens(tokens);
  }
}
impl Field for BaseUrl {
  fn add_to_impl_members(&self, impl_block: &mut syn::ItemImpl) {
    add_to_impl_members(self, "base_url", impl_block);
  }
}

pub struct Headers(Expr);
impl Parse for Headers {
  fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
    Ok(Self(input.parse()?))
  }
}
impl ToTokens for Headers {
  fn to_tokens(&self, tokens: &mut TokenStream2) {
    self.0.to_tokens(tokens);
  }
}
impl Field for Headers {
  fn add_to_impl_members(&self, impl_block: &mut syn::ItemImpl) {
    add_to_impl_members(self, "headers", impl_block);
  }
}

pub struct Body(Expr);
impl Parse for Body {
  fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
    Ok(Self(input.parse()?))
  }
}
impl ToTokens for Body {
  fn to_tokens(&self, tokens: &mut TokenStream2) {
    self.0.to_tokens(tokens);
  }
}
impl Field for Body {
  fn add_to_impl_members(&self, impl_block: &mut syn::ItemImpl) {
    add_to_impl_members(self, "body", impl_block);
  }
}

pub struct Query(Expr);
impl Parse for Query {
  fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
    Ok(Self(input.parse()?))
  }
}
impl ToTokens for Query {
  fn to_tokens(&self, tokens: &mut TokenStream2) {
    self.0.to_tokens(tokens);
  }
}
impl Field for Query {
  fn add_to_impl_members(&self, impl_block: &mut syn::ItemImpl) {
    add_to_impl_members(self, "query", impl_block);
  }
}

pub struct SharedQuery(Expr);
impl Parse for SharedQuery {
  fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
    Ok(Self(input.parse()?))
  }
}
impl ToTokens for SharedQuery {
  fn to_tokens(&self, tokens: &mut TokenStream2) {
    self.0.to_tokens(tokens);
  }
}
impl Field for SharedQuery {
  fn add_to_impl_members(&self, impl_block: &mut syn::ItemImpl) {
    add_to_impl_members(self, "shared_query", impl_block);
  }
}

pub struct BasicAuth(Expr);
impl Parse for BasicAuth {
  fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
    Ok(Self(input.parse()?))
  }
}
impl ToTokens for BasicAuth {
  fn to_tokens(&self, tokens: &mut TokenStream2) {
    self.0.to_tokens(tokens);
  }
}
impl Field for BasicAuth {
  fn add_to_impl_members(&self, impl_block: &mut syn::ItemImpl) {
    add_to_impl_members(self, "basic_auth", impl_block);
  }
}

pub struct BearerAuth(Expr);
impl Parse for BearerAuth {
  fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
    Ok(Self(input.parse()?))
  }
}
impl ToTokens for BearerAuth {
  fn to_tokens(&self, tokens: &mut TokenStream2) {
    self.0.to_tokens(tokens);
  }
}
impl Field for BearerAuth {
  fn add_to_impl_members(&self, impl_block: &mut syn::ItemImpl) {
    add_to_impl_members(self, "bearer_auth", impl_block);
  }
}

pub struct ResponseHandler(Expr);
impl Parse for ResponseHandler {
  fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
    Ok(Self(input.parse()?))
  }
}
impl ToTokens for ResponseHandler {
  fn to_tokens(&self, tokens: &mut TokenStream2) {
    self.0.to_tokens(tokens);
  }
}
impl Field for ResponseHandler {
  fn add_to_impl_members(&self, impl_block: &mut syn::ItemImpl) {
    add_to_impl_members(self, "response_handler", impl_block);
  }
}
