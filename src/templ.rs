// use crate::auth::SessionUser;
use askama::Template;
//
// // #[derive(Template)]
// // #[template(path = "pages/me.html")]
// // pub struct Me<'a> {
// //     pub user: SessionUser,
// // }
//
#[derive(Template)]
#[template(path = "pages/sign_in.html")]
pub struct SignIn {
    pub error: Option<String>,
}

#[derive(Template)]
#[template(path = "pages/register.html")]
pub struct Register<'a> {
    pub sent: &'a bool,
    pub error: &'a Option<String>,
}
//
// // #[derive(Tpl)]
// // #[template(path = "pages/password.hbs")]
// // pub struct Password {
// //     pub email: String,
// //     pub path_id: String,
// //     pub error: Option<String>,
// // }
// //
// // #[derive(Tpl)]
// // #[template(path = "pages/me.hbs")]
// // pub struct Me {
// //     pub user: SessionUser,
// // }
// //
