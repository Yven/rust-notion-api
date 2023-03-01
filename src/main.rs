use notion_api::{notion::{Notion, property::PropertyType, sort::Direction, database::Database}};
use anyhow::{Result, Ok};
use dotenv::dotenv;
use std::env;

fn main() -> Result<()> {
    dotenv().ok();

    let s1 = PropertyType::Status("Status").equals("archive");
    let s2 = PropertyType::MultiSelect("Tag").contains("test");
    let filter = s1.and(s2);

    let mut database = Notion::Databases(env::var("DB_ID")?)
        .filter(filter)
        .sort(PropertyType::Date("Edited time"), Direction::Descending)
        .search::<Database>()?;

    for page in database.page_list.iter_mut() {
        let path = env!("CARGO_MANIFEST_DIR").to_string() + "/" + &page.title + ".md";
        std::fs::write(path, page.content()?)?;
    }

    Ok(())
}
