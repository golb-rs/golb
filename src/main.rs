//TODO:添加 `lib.rs` ，将项目的 `main.rs` 中的一些逻辑分离到此中

//use actix_files::Files;
//use actix_web::App;
use clap::*;
//use indicatif::*;
use anyhow::Result;
use colored::Colorize;
use std::fmt::Display;
use std::{fs, io::Write, thread, time};
use upon::*;

const DEBUG: bool = false;
const CONFIG_STR: &[u8] = include_str!("./config_template.toml").as_bytes();
const MD_STR: &[u8] = include_str!("./helloworld_template.md").as_bytes();
const THEME: &str = "mini"; //TODO:读取配置
#[derive(Debug)]
struct Page {
    name: String,
    content: String,
}
#[derive(Debug)]
struct UserError {
    msg: String, 
}
impl Display for UserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", "User Error".red(), self.msg) //TODO:给 `User Error` 增加颜色
    }
}
impl std::error::Error for UserError {}
impl UserError {
    fn new(msg: String) -> Self {
        UserError { msg }
    }
}

fn parse_config() {}
fn new(m: Option<&str>) -> Result<()> {
    if let Some(post_name) = m {}
    Ok(())
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
    let pages = parse();
    dbg!(&pages);
    let pages = pages.unwrap();
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
    let theme_dir = "./themes/".to_owned() + THEME;

    let mut has_base = false;
    let base = fs::read_to_string(theme_dir.clone() + "/base.html")?;
    let base = engine.compile(&base)?;

    let mut pages = vec![];
    let mut parsed_pages: Vec<Page> = vec![];

    let templates = fs::read_dir(theme_dir)?;
    for page in templates {
        let page = page?;
        if page.file_name() == "base.html" {
            has_base = true;
            continue;
        }
        let name = page.file_name().into_string().unwrap();
        dbg!(&name);
        let template = fs::read_to_string(page.path())?;
        let temp = engine.compile(&template)?;
        let content = temp.render(upon::value! { user: { name: "XYZscratcher" }})?;
        pages.push(Page { name, content });
    }
    if has_base {
        for page in &pages {
            let r = base.render(
                upon::value! { site: { name: "XYZscratcher's blog" },CONTENT:page.content.as_str()},
            )?;
            //let s=r.replace("{{CONTENT}}",page.content.as_str());
            parsed_pages.push(Page {
                name: page.name.clone(),
                content: r,
            })
            //TODO:让模板可导入组件
        }
    }
    //let template = engine.get_template("index.html").unwrap();
    //let result = template.render(upon::value! { user: { name: "John Smith" }})?;

    dbg!(&parsed_pages);
    Ok(parsed_pages)
}
fn server() -> Result<()> {
    //TODO:运行一个静态服务器
    //let app = App::new().service(Files::new("/build", ".").prefer_utf8(true));
    Ok(())
}
fn main() {
    /*let ten_millis = time::Duration::from_millis(1);
    let now = time::Instant::now();*/
    /*std::panic::set_hook(Box::new(|i| {
        eprintln!("{:#?}", i) //FIXME:这里可以只输出错误信息，不必输出其它
    }));*/
    let mut m = command!()
        .subcommands([
            Command::new("init").about(""),
            Command::new("build").about(""),
            Command::new("new")
                .about("")
                .arg(Arg::new("post_name"))
                .arg(
                    Arg::new("draft")
                        .short('d')
                        .long("draft")
                        .action(ArgAction::SetTrue),
                ),
            Command::new("server").about(""),
        ]);
    let h=m.render_help();
    let m=m.get_matches();
    
    let err = match m.subcommand() {
        Some(("init", _)) => init(),
        Some(("build", _)) => build(),
        Some(("server", _)) => server(),
        Some(("new", m)) => new(m.get_one::<String>("post_name").map(|x| x.as_str())),
        None=>{println!("{}",h);Ok(())}
        _ => Ok(()),
    }
    .unwrap_or_else(|e| eprintln!("{}", e));
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
