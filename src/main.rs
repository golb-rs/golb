//TODO:添加 `lib.rs` ，将项目的 `main.rs` 中的一些逻辑分离到此中
use axum::{
    routing::{get, post},
    http::StatusCode,
    response::IntoResponse,
    Json, Router,
};
use std::net::SocketAddr;
//use actix_files::Files;
//use actix_web::App;
use clap::*;
//use indicatif::*;
use anyhow::Result;
use colored::Colorize;
use human_panic::setup_panic;
use markdown::to_html;
use std::fmt::Display;
use std::{fs, io::Write, process};
//use upon; 有用到 upon，但没必要将它引入作用域；另见 https://rust-lang.github.io/rust-clippy/master/index.html#/single_component_path_imports

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
struct AppError {
    kind: String,
    msg: String,
}

impl Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let err = &self.kind;
        let err_text = format!("{} Error:", err).red();
        write!(f, "{} {}", err_text, self.msg)
    }
}
impl std::error::Error for AppError {}
impl AppError {
    fn new(kind: String, msg: String) -> Self {
        AppError { kind, msg }
    }
} //FIXME:`AppError` 目前好像没用？先注释掉
#[warn(dead_code)]
fn parse_config() {}
fn new(post_name: &str) -> Result<()> {
    //if let Some(post_name) = m {
    let fname = post_name.replace(' ', "-").to_ascii_lowercase() + ".md";
    let mut f = fs::File::create("./posts/".to_owned() + &fname)?;
    if let Ok(template) = fs::read_to_string("./posts/_index.md") {
        let t = template.replace("{{title}}", post_name);
        f.write_all(t.as_bytes())?;
    }
    //f.write_all()
    //}
    Ok(())
}
fn init(name: Option<&str>) -> Result<()> {
    let mut dir = String::from("./");
    if let Some(name) = name {
        fs::create_dir_all(name)?;
        dir = format!("./{}/", name);
    }
    let mut dir_builder = fs::DirBuilder::new();
    dir_builder.recursive(true);
    if !DEBUG {
        for i in ["posts", "themes", "build"] {
            dir_builder.create(dir.clone() + i)?;
            if i == "posts" {
                fs::File::create(dir.clone() + "posts/hello_world.md")?.write_all(MD_STR)?;
            }
        }
        let mut f = fs::File::create(dir + "./golb.config.toml")?;
        f.write_all(CONFIG_STR)?;
    };

    Ok(())
}
fn build() -> Result<()> {
    let pages = parse();
    dbg!(&pages);
    let pages = pages?;
    for page in pages {
        //let page=page.unwrap();
        dbg!(&page);
        let mut f = fs::File::create("./build/".to_owned() + &page.name)?;
        f.write_all(page.content.as_str().as_bytes())?;
    }
    Ok(())
}
fn parse() -> Result<Vec<Page>> {
    let engine = upon::Engine::new();

    //dbg!(&engine);
    let theme_dir = "./themes/".to_owned() + THEME;

    let mut has_base = false;
    let base = fs::read_to_string(theme_dir.clone() + "/base.html")?;
    let _post = fs::read_to_string(theme_dir.clone() + "/post.html")?;
    let base = engine.compile(&base)?;

    let mut pages: Vec<Page> = vec![];
    let mut parsed_pages: Vec<Page> = vec![];

    let templates = fs::read_dir(theme_dir.clone())?;
    let posts = fs::read_dir("./posts/")?;
    //TODO:
    //FIXME:page 先不管
    /*for page in templates {
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
    }*/
    for post in posts {
        let p = post?;
        if p.file_name() != "_index.md" {
            let file = fs::read_to_string(&p.path())?;
            let mut flag = file.lines().collect::<Vec<_>>()[0];
            enum FrontMatterType {
                YAML,
                TOML,
                None,
            }
            let mut front_matter_type = FrontMatterType::None; //TODO:解析 Front matter
            let [front_matter, other_content] = {
                //FIXME:Ahhhhhhhhhhhhhhhhhhhh!
                //let flag = flag.unwrap();
                let fmtype = file.split(flag);
                if flag == "---" {
                    front_matter_type = FrontMatterType::YAML;
                } else if flag == "+++" {
                    front_matter_type = FrontMatterType::TOML;
                } else {
                }
                let r = &fmtype.collect::<Vec<_>>()[..];
                dbg!(&r);
                if r.len() <= 2 {
                    [r[1], ""]
                } else {
                    [r[1], r[1]]
                }
            };
            let html = to_html(other_content);
            let post_template = engine
                .compile(fs::read_to_string(theme_dir.clone() + "/post.html")?.as_str())?
                .render(upon::value! {content: html})?;
            let name = p
                .file_name()
                .to_str()
                .unwrap()
                .to_string()
                .replacen(".md", ".html", 1);
            dbg!(&name);
            parsed_pages.push(Page {
                name,
                content: post_template,
            })
        }
    }
    //let template = engine.get_template("index.html").unwrap();
    //let result = template.render(upon::value! { user: { name: "John Smith" }})?;

    dbg!(&parsed_pages);
    Ok(parsed_pages)
}
async fn server() -> Result<()> {
    /*let path = std::env::current_dir()?.join("build/");
    let server = file_serve::ServerBuilder::new(&path).hostname("localhost").port(4096).build();
    println!("Serving {}", path.display());
    println!("See http://{}", server.addr());
    println!("Hit CTRL-C to stop");
    server.serve()?;*/
    /*async fn index()->&'static str{
        "hello"
    }*/
    let app = axum_static::static_router("build/");
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    //tracing::debug!("listening on {}", addr);
    println!("http://{addr}");
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
    Ok(())
    //todo!()
    //Err(AppError::new("No".to_string(), "test".to_string()).into())
}
#[tokio::main]
async fn main() {
    setup_panic!();
    /*let ten_millis = time::Duration::from_millis(1);
    let now = time::Instant::now();*/
    /*std::panic::set_hook(Box::new(|i| {
        eprintln!("{:#?}", i)
    }));*/
    let mut command = command!().subcommands([
        Command::new("init")
            .about("初始化一个博客目录")
            .arg(Arg::new("dir_name")),
        Command::new("build").about("构建静态页面"),
        Command::new("new")
            .about("创建一篇文章")
            .arg(Arg::new("post_name").required(true)),
        Command::new("server")
            .alias("serve")
            .about("Serve the site.")
            .arg(
                Arg::new("draft")
                    .short('d')
                    .long("draft")
                    .action(ArgAction::SetTrue),
            ),
    ]);
    let help_text = command.render_help();
    let matches = command.get_matches();

    match matches.subcommand() {
        Some(("init", m)) => init(m.get_one::<String>("dir_name").map(|x| x.as_str())),
        Some(("build", _)) => build(),
        Some(("server", _)) => server().await,
        Some(("new", m)) => new(m.get_one::<String>("post_name").unwrap()),
        None => {
            println!("{}", help_text);
            Ok(())
        }
        _ => Ok(()),
    }
    /*.unwrap_or_else(|e| {
        eprintln!("{}", e);
        process::exit(1)
    }); */
    .unwrap();
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
