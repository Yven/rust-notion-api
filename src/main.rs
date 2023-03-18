use futures::executor::block_on;
use notion_api::notion::{Notion, property::PropertyType, sort::Direction, database::Database, page::Page, NewImp};
use notion_api::{db_connection, entity};
use anyhow::{Result, Ok};
use dotenv::dotenv;
use std::env;

use log::info;

use env_logger::{fmt::Color, Builder, Env};
use std::io::Write;

fn init_logger() {
    let env = Env::default()
        .filter("LOG_LEVEL")
        .write_style("LOG_STYLE");

    Builder::from_env(env)
        .format(|buf, record| {
            let mut style = buf.style();
            style.set_bg(Color::Yellow).set_bold(true);

            let timestamp = buf.timestamp();

            writeln!(
                buf,
                "My formatted log ({}): {}",
                timestamp,
                style.value(record.args())
            )
        })
        .init();
}


fn main() -> Result<()> {
    dotenv().ok();
    init_logger();

    info!("test logger info");

    let db = block_on(db_connection())?;

    let s1 = PropertyType::Status("Status").equals("publish");
    let s2 = PropertyType::MultiSelect("Tag").contains("test");
    let filter = s1.and(s2);

    let mut database = Notion::Databases(env::var("DB_ID")?)
        .filter(filter)
        .sort(PropertyType::Date("Edited time"), Direction::Descending)
        // .limit(5)
        .search::<Database>()?;

    while database.has_more {
        database.next()?;
    }

    let page_path = env::var("PAGE_SAVE_PATH")?;
    for mut page in database.page_list.into_iter() {
        // println!("{:#?}", page);
        let id = page.id.clone();
        let path = format!("{}/{}.md", page_path.trim_end_matches('/'), page.title);
        std::fs::write(path, page.content()?)?;
        if block_on(entity::is_exist(&db, page.search_property("Slug").unwrap().to_string()))? {
            println!("updating...");
            block_on(entity::update_article(&db, page))?;
        } else {
            println!("creating...");
            block_on(entity::new_article(&db, page))?;
        }
        let page = Notion::Pages(id).update::<Page>(vec![(PropertyType::Status("Status"), "archive")])?;
        println!("Now Page 【{}】 status : {}", page.title, page.search_property("Status").unwrap().to_string());
    }

    Ok(())
}