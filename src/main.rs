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
        .search::<Database>()?;

    for mut page in database.page_list.into_iter() {
        let path = env!("CARGO_MANIFEST_DIR").to_string() + "/" + &page.title + ".md";
        std::fs::write(path, page.content()?)?;
        page.content()?;
        block_on(entity::new_article(&db, page))?;
    }
    Ok(())
}