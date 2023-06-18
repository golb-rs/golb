//use actix_files::Files;
//use actix_web::App;
use clap::*;
use indicatif::*;
use std::{fs, io::Write, thread, time};
use upon::*;
use anyhow::Result;

const DEBUG: bool = false;
const CONFIG_STR: &[u8] = include_str!("./config_template.toml").as_bytes();
const MD_STR: &[u8] = include_str!("./helloworld_template.md").as_bytes();
const THEME: &str = "mini";
#[derive(Debug)]
struct Page {
    name: String,
    content: String,
}

fn init() -> Result<()> {
    let mut dir_builder = fs::DirBuilder::new();
    dir_builder.recursive(true);
    if !DEBUG {
        for i in ["posts", "themes", "build"] {
            dir_builder.create(i);
            if i == "posts" {
                fs::File::create("posts/hello_world.md")?.write_all(MD_STR);
            }
        }
        let mut f = fs::File::create("./golb.config.toml")?;
        f.write_all(CONFIG_STR);
    };
    Ok(())
}
fn build() -> Result<()> {
    let pages = parse().unwrap();
    dbg!(&pages);
    for page in pages {
        //let page=page.unwrap();
        dbg!(&page);
        let mut f = fs::File::create("./build/".to_owned() + &page.name)?;
        f.write_all(page.content.as_str().as_bytes())?;
    }
    Ok(())
}
fn parse() -> Result<Vec<Page>> {
    let mut engine = upon::Engine::new();
    //dbg!(&engine);
    let mut pages = vec![];
    let templates = fs::read_dir("./themes/".to_owned() + THEME)?;
    for page in templates {
        let page = page?;
        let name = page.file_name().into_string().unwrap();
        dbg!(&name);
        let template = fs::read_to_string(page.path())?;
        let temp = engine.compile(&template)?;
        let content = temp.render(upon::value! { user: { name: "John Smith" }})?;
        pages.push(Page { name, content });
    }
    //let template = engine.get_template("index.html").unwrap();
    //let result = template.render(upon::value! { user: { name: "John Smith" }})?;

    dbg!(&pages);
    Ok(pages)
}
fn server()-> Result<()>{
    //let app = App::new().service(Files::new("/build", ".").prefer_utf8(true));
    Ok(())
}
fn main() {
    /*let ten_millis = time::Duration::from_millis(1);
    let now = time::Instant::now();*/

    let m = command!()
        .subcommands([
            Command::new("init").about(""),
            Command::new("build").about(""),
            Command::new("new").about(""),
            Command::new("server").about(""),
        ])
        .get_matches();

    match m.subcommand() {
        Some(("init", _)) => init(),
        Some(("build", _)) => build(),
        Some(("server", _)) => server(),
        _ => Ok(()),
    };
    /*let bar = ProgressBar::new(1000);
    bar.set_style(
        ProgressStyle::with_template(
            "[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}",
        )
        .unwrap()
        .progress_chars("=>#"),
    );
    for _ in 0..1000 {
        bar.inc(1);
        thread::sleep(ten_millis);
    }
    bar.finish();*/
}
