use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::ToTokens;
use syn::{Expr, Ident, ImplItemFn, Meta};

use super::fields::{BaseUrl, BasicAuth, BearerAuth, Body, Client, Headers, Query, SharedQuery};

pub struct RequestFunction {
  pub fn_item: ImplItemFn,
  pub request: Request,
}
impl RequestFunction {
  pub fn new_from_attrib(method: Method, args: TokenStream, item: TokenStream) -> Self {
    let fn_item = syn::parse(item).expect("Failed to parse function");
    let request = Request::new(method, args, &fn_item);

    Self { fn_item, request }
  }

  pub fn generate(&self) -> TokenStream2 {
    let base_url = &self.request.base_url;
    let path = &self.request.path;
    let url = quote::quote! { format!("{}{}", #base_url, #path) };
    let client = &self.request.client;
    let method = match self.request.method {
      Method::Get => quote::quote! { get },
      Method::Post => quote::quote! { post },
      Method::Put => quote::quote! { put },
      Method::Delete => quote::quote! { delete },
    };
    let mut init = Default::default();
    let mut call = quote::quote! { #client.#method(#url) };
    if let Some(headers) = &self.request.headers {
      init = quote::quote! {
        #init
        let headers = HashMap::from_iter(#headers.iter().map(|(k, v)| (k.to_string(), v.to_string())));
        let headers = HeaderMap::try_from(&headers).unwrap();
      };
      call = quote::quote! {
        #call.headers(headers)
      };
    }
    if let Some(body) = &self.request.body {
      let style = match self.request.body_type {
        BodyType::Raw => quote::quote! { body },
        BodyType::Json => quote::quote! { json },
        BodyType::FormUrlEncoded => quote::quote! { form },
        BodyType::Multipart => quote::quote! { multipart },
      };
      call = quote::quote! { #call.#style(#body) };
    }
    if let Some(query) = &self.request.query {
      call = quote::quote! { #call.query(#query) };
    }
    if let Some(shared_query) = &self.request.shared_query {
      call = quote::quote! { #call.query(#shared_query) };
    }
    if let Some(bearer_auth) = &self.request.bearer_auth {
      call = quote::quote! { #call.bearer_auth(#bearer_auth) };
    } else if let Some(basic_auth) = &self.request.basic_auth {
      call = quote::quote! { #call.basic_auth(#basic_auth) };
    }

    call = quote::quote! { #call.send()? };
    let block = quote::quote! {
      {
        #init
        Ok(#call.json()?)
      }
    };
    let mut func = self.fn_item.clone();
    func.block = syn::parse2(block).expect("Failed to parse block");
    func.into_token_stream()
  }
}

pub struct Request {
  pub client: Client,
  pub method: Method,
  pub base_url: BaseUrl,
  pub path: Expr,
  pub headers: Option<Headers>,
  pub body: Option<Body>,
  pub body_type: BodyType,
  pub query: Option<Query>,
  pub shared_query: Option<SharedQuery>,
  pub basic_auth: Option<BasicAuth>,
  pub bearer_auth: Option<BearerAuth>,
}
impl Request {
  pub fn new(method: Method, path: TokenStream, func: &ImplItemFn) -> Self {
    let path = syn::parse(path).expect("Failed to parse path");
    let attrs = &func.attrs;

    let mut client = None;
    let mut base_url = None;
    let mut headers = None;
    let mut body = None;
    let mut query = None;
    let mut shared_query = None;
    let mut basic_auth = None;
    let mut bearer_auth = None;
    let mut body_type = BodyType::Raw;

    for attr in attrs.iter().rev() {
      let list = match &attr.meta {
        Meta::List(list) => list,
        _ => continue,
      };
      let path = list.path.clone();

      match path.get_ident().map(Ident::to_string).unwrap_or_default().as_str() {
        "client" => client = Some(list.tokens.clone()),
        "base_url" => base_url = Some(list.tokens.clone()),
        "headers" => headers = Some(list.tokens.clone()),
        "body" => body = Some(list.tokens.clone()),
        "query" => query = Some(list.tokens.clone()),
        "shared_query" => shared_query = Some(list.tokens.clone()),
        "basic_auth" => basic_auth = Some(list.tokens.clone()),
        "bearer_auth" => bearer_auth = Some(list.tokens.clone()),

        "raw" => body_type = BodyType::Raw,
        "json" => body_type = BodyType::Json,
        "form_url_encoded" => body_type = BodyType::FormUrlEncoded,
        "multipart" => body_type = BodyType::Multipart,
        _ => continue,
      }
    }
    let client = client.expect("A `client` attribute is required");
    let base_url = base_url.expect("A `base_url` attribute is required");

    let client = syn::parse2(client).expect("Failed to parse client");
    let base_url = syn::parse2(base_url).expect("Failed to parse base_url");
    let headers = headers.map(|h| syn::parse2(h).expect("Failed to parse headers"));
    let body = body.map(|b| syn::parse2(b).expect("Failed to parse body"));
    let query = query.map(|q| syn::parse2(q).expect("Failed to parse query"));
    let shared_query = shared_query.map(|q| syn::parse2(q).expect("Failed to parse shared_query"));
    let basic_auth = basic_auth.map(|a| syn::parse2(a).expect("Failed to parse basic_auth"));
    let bearer_auth = bearer_auth.map(|a| syn::parse2(a).expect("Failed to parse bearer_auth"));

    Self {
      client,
      method,
      base_url,
      path,
      headers,
      body,
      body_type,
      query,
      shared_query,
      basic_auth,
      bearer_auth,
    }
  }
}

pub enum Method {
  Get,
  Post,
  Put,
  Delete,
}

pub enum BodyType {
  Raw,
  Json,
  FormUrlEncoded,
  Multipart,
}
