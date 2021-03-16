use crate::content::{Content, CreateContent, CreateSampleContent};
use deadpool_postgres::Client;
use std::io;
use tokio_pg_mapper::FromTokioPostgresRow;

// CORE CRUD

//TODO configure .env for db shema name

// Decide wether to return id or return all fields from insert sql query . if return ID, insert id in function argument.
// shift id in db tables to the top so we can skip it when not needed

pub async fn content_add(
    client: &Client,
    selfobj: CreateContent,
) -> Result<CreateContent, io::Error> {
    let statement = client.prepare("INSERT INTO public.content
   (type_id, title, summary, details, submitted_date, modified_date)
    VALUES ($0, $1, $2, $3, $4, $5) RETURNING type_id, title, summary, details, submitted_date, modified_date").await.unwrap();

    client
        .query(
            &statement,
            &[
                &selfobj.type_id,
                &selfobj.title,
                &selfobj.summary,
                &selfobj.details,
                &selfobj.submitted_date,
                &selfobj.modified_date,
            ],
        )
        .await
        .expect("Error creating content")
        .iter()
        .map(|row| CreateContent::from_row_ref(row).unwrap())
        .collect::<Vec<CreateContent>>()
        .pop()
        .ok_or(io::Error::new(
            io::ErrorKind::Other,
            "Error creating content tables",
        ))
}

pub async fn contentsample_add(
    client: &Client,
    selfobj: CreateSampleContent,
) -> Result<CreateSampleContent, io::Error> {
    let statement = client
        .prepare(
            "INSERT INTO public.content
   (type_id, title, summary, details)
    VALUES ($1, $2, $3, $4) RETURNING type_id, title, summary, details",
        )
        .await
        .unwrap();

    client
        .query(
            &statement,
            &[
                &selfobj.type_id,
                &selfobj.title,
                &selfobj.summary,
                &selfobj.details,
            ],
        )
        .await
        .expect("Error creating content")
        .iter()
        .map(|row| CreateSampleContent::from_row_ref(row).unwrap())
        .collect::<Vec<CreateSampleContent>>()
        .pop()
        .ok_or(io::Error::new(
            io::ErrorKind::Other,
            "Error creating content tables",
        ))
}

// TODO populate fields

pub async fn content_list(client: &Client) -> Result<Vec<Content>, io::Error> {
    let statement = client
        .prepare("select * from public.content order by id desc")
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

pub async fn content_id(client: &Client, id_content: i32) -> Result<Content, io::Error> {
    let statement = client
        .prepare("select * from public.content where id = $1")
        .await
        .unwrap();

    let maybe_content = client
        .query_opt(&statement, &[&id_content])
        .await
        .expect("Error fetching content ")
        .map(|row| Content::from_row_ref(&row).unwrap());

    match maybe_content {
        Some(content) => Ok(content),
        None => Err(io::Error::new(io::ErrorKind::NotFound, "Not found")),
    }
}

pub async fn content_from_type(
    client: &Client,
    id_content: i32,
) -> Result<Vec<Content>, io::Error> {
    let statement = client
        .prepare("select * from public.content where type_id = $1")
        .await
        .unwrap();

    let maybe_content = client
        .query(&statement, &[&id_content])
        .await
        .expect("Error fetching content ")
        .iter()
        .map(|row| Content::from_row_ref(&row).unwrap())
        .collect::<Vec<Content>>();
    Ok(maybe_content)
}

//TODO take into account ID position

pub async fn content_update(client: &Client, id: i32, mdl: CreateContent) -> Result<(), io::Error> {
    let statement = client.prepare("update public.content set (type_id, title, summary, details, modified_date) = ($1, $2, $3, $4, $5) where id = $6").await.unwrap();

    let result = client
        .execute(
            &statement,
            &[
                &mdl.type_id,
                &mdl.title,
                &mdl.summary,
                &mdl.details,
                &mdl.modified_date,
                &id,
            ],
        )
        .await
        .expect("Error updating content");

    match result {
        ref updated if *updated == 1 => Ok(()),
        _ => Err(io::Error::new(io::ErrorKind::Other, "Failed to check list")),
    }
}

pub async fn content_delete(client: &Client, content_id: i32) -> Result<(), io::Error> {
    let statement = client
        .prepare("DELETE FROM public.content WHERE id = $1")
        .await
        .unwrap();

    client.execute(&statement, &[&content_id]).await.unwrap();
    Ok(())
}

// END OF CORE CRUD
