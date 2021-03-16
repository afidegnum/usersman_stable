use askama::Template;
use serde::{Deserialize, Serialize};
use tokio_pg_mapper_derive::PostgresMapper;

#[derive(Serialize, Clone, Deserialize, PostgresMapper, Debug)]
#[pg_mapper(table = "menus")]
pub struct Menu {
    pub id: i32,
    pub name: String,
    pub title: String,
}

#[derive(Serialize, Clone, Deserialize, PostgresMapper, Debug)]
#[pg_mapper(table = "menus")]
pub struct Content {
    pub id: i32,
    pub title: String,
    pub summary: String,
    pub details: String,
}

#[derive(Serialize, Clone, Deserialize, PostgresMapper, Debug)]
#[pg_mapper(table = "content")]
pub struct ShortContent {
    pub id: i32,
    pub title: String,
    pub summary: String,
}

#[derive(Template)]
#[template(path = "pages/user.html")]
struct UserTemplate<'a> {
    name: &'a str,
    text: &'a str,
    url: &'a str,
    menus: &'a Vec<Menu>,
}

#[derive(Template, Clone, Debug)]
#[template(path = "pages/layout.html")]
pub(crate) struct LayoutTemplate<'a> {
    pub(crate) url: &'a str,
    pub(crate) menus: Vec<Menu>,
    pub(crate) shortcontents: Vec<ShortContent>,
}

#[derive(Template, Debug)]
#[template(path = "pages/index.html")]
pub(crate) struct HomeTemplate<'a> {
    pub(crate) top: LayoutTemplate<'a>,
    pub(crate) contents: Vec<Content>,
}

#[derive(Template, Debug)]
#[template(path = "pages/blog.html")]
pub(crate) struct BlogTemplate<'a> {
    pub(crate) top: LayoutTemplate<'a>,
}

#[derive(Template, Debug)]
#[template(path = "pages/about.html")]
pub(crate) struct AboutTemplate<'a> {
    pub(crate) top: LayoutTemplate<'a>,
}

#[derive(Template, Debug)]
#[template(path = "pages/contact.html")]
pub(crate) struct ContactTemplate<'a> {
    pub(crate) top: LayoutTemplate<'a>,
}

#[derive(Template, Debug)]
#[template(path = "pages/page.html")]
pub(crate) struct PageTemplate<'a> {
    pub(crate) top: LayoutTemplate<'a>,
}

#[derive(Template, Debug)]
#[template(path = "pages/default.html")]
pub(crate) struct DefaultTemplate<'a> {
    pub(crate) top: LayoutTemplate<'a>,
}

#[derive(Template)]
#[template(path = "pages/not_found.html")]
struct NotFound;
