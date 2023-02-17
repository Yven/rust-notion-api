use notion_api::notion::database;
use notion_api::notion::{term, sort, property};

fn main() {
    let s1 = property::PropertyType::Status("Status".to_string()).equals("archive");
    let s2 = property::PropertyType::MultiSelect("Tag".to_string()).contains("test");
    let filter = s1.and(s2);

    let sort = sort::Sort::new(vec![
        ("Edited time".to_string(), sort::Direction::Descending)
    ]);

    let body = term::ReqBody::new(filter, sort);
    // let request = notion::Request::new(&key);
    // let _response = request.query(notion::NotionModule::Databases, &db_id, body);
    let mut db = database::Database::from_remote(notion_api::CONFIG_MAP.get("db_id").unwrap(), body);
    db.page_list[0].content();
}
