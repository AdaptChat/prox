use axum::{extract::Query, http::HeaderMap, response::IntoResponse};
use mime::{Mime, IMAGE};
use reqwest::header::CONTENT_TYPE;

use crate::{error::Result, get_client};

pub async fn proxy(Query(url): Query<String>) -> Result<impl IntoResponse> {
    let resp = get_client().get(url).send().await?;

    let content_type = resp
        .headers()
        .get(CONTENT_TYPE)
        .ok_or_else(|| {
            (
                resp.url().to_string(),
                resp.status().as_u16(),
                "Missing content type",
            )
        })?
        .to_str()
        .map_err(|_| {
            (
                resp.url().to_string(),
                resp.status().as_u16(),
                "Content type invalid",
            )
        })?;
    let mime = content_type.parse::<Mime>().map_err(|_| {
        (
            resp.url().to_string(),
            resp.status().as_u16(),
            "Content type invalid",
        )
    })?;

    if mime.type_() == IMAGE {
        Ok((
            {
                let mut headers = HeaderMap::with_capacity(1);
                headers.append(
                    "Content-Type",
                    content_type.parse().map_err(|_| {
                        (
                            resp.url().to_string(),
                            resp.status().as_u16(),
                            "Content type invalid",
                        )
                    })?,
                );

                headers
            },
            resp.bytes().await?,
        ))
    } else {
        Err((
            resp.url().to_string(),
            resp.status().as_u16(),
            "Unable to proxy given content type",
        )
            .into())
    }
}
