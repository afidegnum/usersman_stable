use crate::pages::{Content, Menu, ShortContent};
use deadpool_postgres::{Client, Pool};
use std::io;
use tokio_pg_mapper::FromTokioPostgresRow;

pub async fn menu_list(client: &Client) -> Result<Vec<Menu>, io::Error> {
    let statement = client
        .prepare("select id, name, title  from public.menu order by id desc")
        .await
        .unwrap();

    let menu_list = client
        .query(&statement, &[])
        .await
        .expect("Error getting author lists")
        .iter()
        .map(|row| Menu::from_row_ref(row).unwrap())
        .collect::<Vec<Menu>>();

    Ok(menu_list)
}

pub async fn content_select(
    client: &Client,
    id_content: &str,
) -> Result<Vec<ShortContent>, io::Error> {
    let statement = client
        // .prepare("select id, title, summary from public.content where menu_id = $1")
    // .prepare("select c.id, c.title, c.summary from public.menus m join content c on m.id = c.menu_id where m.title = $1")
    // .prepare("select c.id, c.title, c.summary from public.menus m join content c on m.id = c.menu_id where m.title = $1")
	.prepare("select p.id, p.title, p.summary from public.menu m join pages p on m.id = p.menu_id where m.title = $1")
        .await
        .unwrap();

    let maybe_content = client
        .query(&statement, &[&id_content])
        .await
        .expect("Error fetching content ")
        .iter()
        .map(|row| ShortContent::from_row_ref(&row).unwrap())
        .collect::<Vec<ShortContent>>();
    Ok(maybe_content)
}

pub async fn content_list(client: &Client) -> Result<Vec<Content>, io::Error> {
    let statement = client
        .prepare("select id, title , summary, details  from public.menu order by id desc")
        .await
        .unwrap();

    let content_list = client
        .query(&statement, &[])
        .await
        .expect("Error getting author lists")
        .iter()
        .map(|row| Content::from_row_ref(row).unwrap())
        .collect::<Vec<Content>>();

    Ok(content_list)
}
