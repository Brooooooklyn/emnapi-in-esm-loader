#![deny(clippy::all)]

use std::collections::HashMap;

use napi::bindgen_prelude::*;
use napi_derive::napi;

#[napi(object)]
#[derive(Debug)]
pub struct ResolveContext {
  /// Export conditions of the relevant `package.json`
  pub conditions: Vec<String>,
  /// An object whose key-value pairs represent the assertions for the module to import
  pub import_attributes: HashMap<String, String>,

  #[napi(js_name = "parentURL")]
  pub parent_url: Option<String>,
}

#[napi(object)]
pub struct ResolveFnOutput {
  pub format: Either3<String, Undefined, Null>,
  pub short_circuit: Option<bool>,
  pub url: String,
  pub import_attributes: Option<Either<HashMap<String, String>, Null>>,
}

#[napi]
pub fn resolve(
  specifier: String,
  context: ResolveContext,
  next_resolve: Function<
    (String, Option<ResolveContext>),
    Either<ResolveFnOutput, PromiseRaw<ResolveFnOutput>>,
  >,
) -> Result<Either<ResolveFnOutput, PromiseRaw<ResolveFnOutput>>> {
  println!("resolve: specifier = {:?}", specifier);
  next_resolve.call((specifier, Some(context)))
}
