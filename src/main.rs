use futures::executor::block_on;
use notion_api::notion::{Notion, property::PropertyType, sort::Direction, database::Database};
use notion_api::{db_connection, entity};
use anyhow::{Result, Ok};
use dotenv::dotenv;
use std::env;


fn main() -> Result<()> {
    dotenv().ok();

    let db = block_on(db_connection())?;

    let s1 = PropertyType::Status("Status").equals("archive");
    let s2 = PropertyType::MultiSelect("Tag").contains("test");
    let filter = s1.and(s2);

    let database = Notion::Databases(env::var("DB_ID")?)
        .filter(filter)
        .sort(PropertyType::Date("Edited time"), Direction::Descending)
        // .limit(5)
        .search::<Database>()?;

    // while database.has_more {
    //     database.next()?;
    // }

    for mut page in database.page_list.into_iter() {
        let path = env!("CARGO_MANIFEST_DIR").to_string() + "/" + &page.title + ".md";
        std::fs::write(path, page.content()?)?;
        // println!("{:#?}", page);
        if block_on(entity::is_exist(&db, page.search_property("Slug").unwrap().to_string()))? {
            println!("update");
            block_on(entity::update_article(&db, page))?;
        } else {
            println!("create");
            block_on(entity::new_article(&db, page))?;
        }
    }

    Ok(())
}