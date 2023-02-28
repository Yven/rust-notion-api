use futures::executor::block_on;
// use notion_api::{notion::{Notion, property::PropertyType, sort::Direction, database::Database}, CONFIG_MAP};
use anyhow::{Result, Ok};
use notion_api::{db_connection, db_contents};
use sea_orm::{Set, entity::EntityTrait};

fn main() -> Result<()> {
    let db = block_on(db_connection())?;

    let articles = db_contents::ActiveModel {
        title: Set("test".to_owned()),
        ..Default::default()
    };

    // let res: InsertResult = Fruit::insert_many([apple, orange]).exec(db).await?;
    let res = block_on(async {
        db_contents::Entity::insert_many([articles]).exec(&db).await
    })?;

    println!("{:#?}", res);

    // let s1 = PropertyType::Status("Status").equals("archive");
    // let s2 = PropertyType::MultiSelect("Tag").contains("test");
    // let filter = s1.and(s2);

    // let mut database = Notion::Databases(CONFIG_MAP.get("db_id").unwrap().to_string())
    //     .filter(filter)
    //     .sort(PropertyType::Date("Edited time"), Direction::Descending)
    //     .search::<Database>()?;

    // for page in database.page_list.iter_mut() {
    //     let path = env!("CARGO_MANIFEST_DIR").to_string() + "/" + &page.title + ".md";
    //     std::fs::write(path, page.content()?)?;
    // }

    Ok(())
}
