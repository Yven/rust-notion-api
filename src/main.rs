use notion_api::{notion::{Notion, property::PropertyType, sort, database::Database}, CONFIG_MAP};

fn main() {
    let s1 = PropertyType::Status("Status".to_string()).equals("archive");
    let s2 = PropertyType::MultiSelect("Tag".to_string()).contains("test");
    let filter = s1.and(s2);

    let database = Notion::Databases(CONFIG_MAP.get("db_id").unwrap().to_string())
        .filter(filter)
        .sort(vec![
            ("Edited time".to_string(), sort::Direction::Descending)
        ]).search::<Database>();

    // for page in database.page_list.iter() {
    //     let property = page.find();
    //     let content = page.fulltext();

    //     content.markdown(page.title);
    //     content.html();
    // }

    let content = database.unwrap().page_list[0].content().unwrap();

    let path = env!("CARGO_MANIFEST_DIR").to_string() + "/output.md";
    match std::fs::write(path, content) {
        Ok(_) => (),
        Err(e) => println!("{:#?}", e),
    }
}
