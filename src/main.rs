use notion_api::{notion::{Notion, property::PropertyType, sort, database::Database}, CONFIG_MAP};

fn main() {
    let database = Notion::Databases(CONFIG_MAP.get("db_id").unwrap().to_string())
        .filter(PropertyType::Status("Status".to_string()).equals("Published").and(PropertyType::People("author".to_string()).contains("Yven")))
        .sort(vec![("Edited Time".to_string(), sort::Direction::Descending)])
        .search::<Database>();

    // for page in database.page_list.iter() {
    //     let property = page.find();
    //     let content = page.fulltext();

    //     content.markdown(page.title);
    //     content.html();
    // }

    // let s1 = property::PropertyType::Status("Status".to_string()).equals("archive");
    // let s2 = property::PropertyType::MultiSelect("Tag".to_string()).contains("test");
    // let filter = s1.and(s2);

    // let sort = sort::Sort::new(vec![
    //     ("Edited time".to_string(), sort::Direction::Descending)
    // ]);

    // let body = term::ReqBody::new(filter, sort);
    // let mut db = database::Database::from_remote(notion_api::CONFIG_MAP.get("db_id").unwrap(), body);
    // let content = db.page_list[0].content();

    // let path = env!("CARGO_MANIFEST_DIR").to_string() + "/output.md";
    // match std::fs::write(path, content) {
    //     Ok(_) => (),
    //     Err(e) => println!("{:#?}", e),
    // }
}
