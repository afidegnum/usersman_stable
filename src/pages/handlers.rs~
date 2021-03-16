use crate::pages::{
    content_list, content_select, menu_list, AboutTemplate, BlogTemplate, ContactTemplate,
    DefaultTemplate, HomeTemplate, LayoutTemplate, PageTemplate,
};
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use deadpool_postgres::{Client, Pool};

use askama::Template;
use std::future::Future;

#[derive(Debug)]
enum TemplateType<'a> {
    HomeTemplate(HomeTemplate<'a>),
    BlogTemplate(BlogTemplate<'a>),
    AboutTemplate(AboutTemplate<'a>),
    ContactTemplate(ContactTemplate<'a>),
    PageTemplate(PageTemplate<'a>),
    DefaultTemplate(DefaultTemplate<'a>),
}

impl LayoutTemplate<'_> {
    async fn get_specific(&self, db_pool: web::Data<Pool>, kind: &str) -> TemplateType<'_> {
        let client: Client = db_pool
            .get()
            .await
            .expect("Error connecting to the database");

        match kind {
            "home" => TemplateType::HomeTemplate(HomeTemplate {
                top: self.clone(),
                contents: content_list(&client).await.expect("error loading contents"),
            }),
            "blog" => TemplateType::BlogTemplate(BlogTemplate { top: self.clone() }),
            "about" => TemplateType::AboutTemplate(AboutTemplate { top: self.clone() }),
            "contact" => TemplateType::ContactTemplate(ContactTemplate { top: self.clone() }),
            "page" => TemplateType::PageTemplate(PageTemplate { top: self.clone() }),
            _ => TemplateType::DefaultTemplate(DefaultTemplate { top: self.clone() }),
        }
    }
}

pub async fn index(
    req: HttpRequest,
    db_pool: web::Data<Pool>,
) -> Result<impl Responder, actix_web::Error> {
    // let path = id_path.into_inner();
    let client: Client = db_pool
        .get()
        .await
        .expect("Error connecting to the database");

    // let result = short_content_from_menu(&client, path.0).await?;
    // url.set_query(Some("q=asdf"));
    // let rurl = req.url_for_static("home").unwrap();

    let menu = req.match_info().get("content").unwrap_or("home");

    let rurl = req.url_for("content", &[&menu]).unwrap();

    let s = LayoutTemplate {
        url: rurl.as_ref(),
        menus: menu_list(&client).await?,
        shortcontents: content_select(&client, &menu).await?,
    };

    let h = HomeTemplate {
        contents: content_list(&client).await?,
        top: s,
    };

    // &s.get_specific("page");

    // let name = req.match_name().unwrap();
    // if menu != "favicon.ico" {let y = LayoutTemplate::get_specific(&s, &menu); };

    // if menu != "favicon.ico" { y = LayoutTemplate::get_specific(&s, &menu); }

    let y = if menu != "favicon.ico" {
        Some(LayoutTemplate::get_specific(&s, db_pool, &menu).await)
    } else {
        None
    };

    // println!("path: {:?} uri: {:?} req: {:?} menu: {:?}", req.path(), req.uri(), &rurl, &y);
    println!(
        "path: {:?} uri: {:?} req: {:?} {:?}",
        req.path(),
        req.uri(),
        &rurl,
        &menu
    );
    // println!("Name: {:#?} ",  &y);
    // let y = LayoutTemplate::get_specific(&s, &menu);

    // if let TemplateType::HomeTemplate(y) = y{
    //     Ok(HttpResponse::Ok().content_type("text/html").body(y.top.render().unwrap()))
    // } else {
    //     unimplemented!()
    // }

    match y {
        Some(TemplateType::HomeTemplate(y)) => {
            // println!("T: {:#?} ", &y);
            Ok(HttpResponse::Ok()
                .content_type("text/html")
                .body(y.render().unwrap()))
        }
        Some(TemplateType::AboutTemplate(y)) => {
            // println!("T: {:#?} ", &y);
            Ok(HttpResponse::Ok()
                .content_type("text/html")
                .body(y.render().unwrap()))
        }
        Some(TemplateType::ContactTemplate(y)) => {
            // println!("T: {:#?} ", &y);
            Ok(HttpResponse::Ok()
                .content_type("text/html")
                .body(y.render().unwrap()))
        }

        Some(TemplateType::PageTemplate(y)) => {
            // println!("T: {:#?} ", &y);
            Ok(HttpResponse::Ok()
                .content_type("text/html")
                .body(y.render().unwrap()))
        }

        Some(TemplateType::BlogTemplate(y)) => {
            // println!("T: {:#?} ", &y);
            Ok(HttpResponse::Ok()
                .content_type("text/html")
                .body(y.render().unwrap()))
        }
        Some(TemplateType::DefaultTemplate(y)) => {
            // println!("T: {:#?} ", &y);
            Ok(HttpResponse::Ok()
                .content_type("text/html")
                .body(y.render().unwrap()))
        }
        None => {
            println!("T: {:#?} M: {:#?}  ", &y, &menu);
            Ok(HttpResponse::Ok()
                .content_type("text/html")
                .body(s.render().unwrap()))
        }
    }
}
