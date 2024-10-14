use std::fs;

use tidal_rs::api::Page;

fn main() {
  let json = fs::read_to_string("./page.json").expect("Please create a valid page.json at the project root");

  let de = &mut serde_json::Deserializer::from_str(&json);
  let result: Result<Page, _> = serde_path_to_error::deserialize(de);

  println!("{:#?}", result);
}
