use notion_api::notion;
use notion_api::notion::term::*;

fn main() {
    let path = env!("CARGO_MANIFEST_DIR").to_string() + "/secret.json";
    let config = std::fs::read_to_string(path).unwrap();
    let config = serde_json::from_str::<serde_json::Value>(&config).unwrap();
    let key = config["key"].as_str().unwrap().to_string();
    let db_id = config["db_id"].as_str().unwrap().to_string();

    let s1 = PropertyType::Select("Status".to_string()).does_not_equal("conception");
    let s2 = PropertyType::Select("Status".to_string()).does_not_equal("edit");
    let s3 = PropertyType::People("Author".to_string()).equals("06a0361f-02e7-4293-8263-2ebdd8569629");
    // s3 || (s1 && s2)
    // ||: s3, (&&: s1, s2)
    let filter = s3.or(s1.and(s2));

    println!("{}", filter.build_str());
    println!("{:#?}", filter);

    let request = notion::Request::new(&key);
    let body = ReqBody::new(filter, SortMap::new());
    let _response = request.query(notion::NotionModule::Databases, &db_id, body).unwrap();
    // println!("{:#?}", response);
}
