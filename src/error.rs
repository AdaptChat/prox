use axum::{response::IntoResponse, Json};

#[derive(serde::Serialize)]
pub struct Error {
    url: String,
    status: u16,
    cause: String,
}

pub type Result<T> = std::result::Result<T, Error>;

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        Json(self).into_response()
    }
}

impl From<reqwest::Error> for Error {
    fn from(req: reqwest::Error) -> Self {
        Self {
            url: req
                .url()
                .map(|x| x.to_string())
                .unwrap_or_else(|| "N/A".to_string()),
            status: req.status().map(|x| x.into()).unwrap_or(0),
            cause: format!("{req}"),
        }
    }
}

impl<T: ToString, R: ToString> From<(T, u16, R)> for Error {
    fn from((url, status, cause): (T, u16, R)) -> Self {
        Self {
            url: url.to_string(),
            status,
            cause: cause.to_string(),
        }
    }
}
