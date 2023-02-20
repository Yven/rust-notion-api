use notion_api::notion::database;
use notion_api::notion::{term, sort, property};

fn main() {
    // let page_list = notion::Notion::Databases(id)
    //     .filter(Property::Status(“Status”).equals(“Published”).and(Property::Author(“author”).contains(“Yven”))
    //     .sort(Property::EditTime(“Edit Time”).desc())
    //     .search();

    // let property = page_list[i].find();
    // let content = page_list[i].fulltext();

    // content.markdown(page_list[i].title);
    // content.html();

    let s1 = property::PropertyType::Status("Status".to_string()).equals("archive");
    let s2 = property::PropertyType::MultiSelect("Tag".to_string()).contains("test");
    let filter = s1.and(s2);

    let sort = sort::Sort::new(vec![
        ("Edited time".to_string(), sort::Direction::Descending)
    ]);

    let body = term::ReqBody::new(filter, sort);
    let mut db = database::Database::from_remote(notion_api::CONFIG_MAP.get("db_id").unwrap(), body);
    let content = db.page_list[0].content();

    let path = env!("CARGO_MANIFEST_DIR").to_string() + "/output.md";
    match std::fs::write(path, content) {
        Ok(_) => (),
        Err(e) => println!("{:#?}", e),
    }
}
