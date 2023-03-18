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

    let log_path = format!("{}/{}.log", env::var("LOG_PATH").unwrap(), chrono::Utc::now().format("%Y%m%d"));
    let path = std::path::Path::new(&log_path);
    if path.try_exists().is_err() || !path.try_exists().unwrap() {
        std::fs::create_dir_all(path.parent().unwrap()).expect("Create Log Path Feiled");
        std::fs::File::create(log_path.clone()).expect("Create Log File Feiled");
    }

    Builder::from_env(env)
        .format(|buf, record| {
            let mut style = buf.style();
            style.set_color(Color::Yellow).set_bold(true);

            writeln!(
                buf,
                "[{}] 文件 {} 中第 {} 行({}):\n{}\n",
                style.value(record.level()),
                style.value(record.file().unwrap()),
                style.value(record.line().unwrap()),
                chrono::Utc::now().with_timezone(&chrono::FixedOffset::east_opt(8 * 3600).unwrap()).format("%Y-%m-%d %H:%M:%S"),
                record.args(),
            )
        })
        .target(env_logger::Target::Pipe(Box::new(std::fs::File::options().append(true).open(log_path).expect("Can't create file"))))
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