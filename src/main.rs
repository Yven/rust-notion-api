use notion_api::notion;
use notion_api::notion::term::*;

fn main() {
    let path = env!("CARGO_MANIFEST_DIR").to_string() + "/secret.json";
    let config = std::fs::read_to_string(path).unwrap();
    let config = serde_json::from_str::<serde_json::Value>(&config).unwrap();
    let key = config["key"].as_str().unwrap().to_string();
    let db_id = config["db_id"].as_str().unwrap().to_string();

    let s1 = PropertyType::Status("Status".to_string()).does_not_equal("conception");
    let s2 = PropertyType::Status("Status".to_string()).does_not_equal("edit");
    // s3 || (s1 && s2)
    // ||: s3, (&&: s1, s2)
    let filter = s1.and(s2);

    let r1 = SortMap::new("Edited time".to_string(), Direction::Descending);

    let body = ReqBody::new(filter, vec![r1]);
    println!("{}", body);

    let request = notion::Request::new(&key);
    let _response = request.query(notion::NotionModule::Databases, &db_id, body);

    match _response {
        Ok(r) => println!("{:#?}", r),
        Err(e) => println!("{:#?}", e)
    }
}
