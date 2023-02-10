use notion_api::notion;
use notion_api::notion::term::*;

fn main() {
    let path = env!("CARGO_MANIFEST_DIR").to_string() + "/secret.json";
    let config = std::fs::read_to_string(path).unwrap();
    let config = serde_json::from_str::<serde_json::Value>(&config).unwrap();
    let key = config["key"].as_str().unwrap().to_string();
    let db_id = config["db_id"].as_str().unwrap().to_string();

    let request = notion::Request::new(&key);
    let body = ReqBody::new(FilterMap::new(), SortMap::new());
    let response = request.query(notion::NotionModule::Databases, &db_id, body).unwrap();

    println!("{:#?}", response);
}
