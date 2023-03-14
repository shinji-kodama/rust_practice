// NotFoundとOtherの2値を持つ列挙型として定義
#[derive(thiserror::Error, Debug)]
pub enum ApiError {
    #[error("Post not found")]
    NotFound,
    #[error(transparent)]
    Other(anyhow::Error),
}

// 定義の記述を楽にするための関数的マクロ
// 使うクレートから返る可能性のあるエラー型からApiError::Otherに変換するためのFromトレイトの実装
macro_rules! impl_from_trait {
    ($etype: ty) => {
        impl From<$etype> for ApiError {
            fn from(e: $etype) -> Self {
                ApiError::Other(anyhow::anyhow!(e))
            }
        }
    };
}

impl_from_trait!(diesel::r2d2::Error);
impl_from_trait!(diesel::r2d2::PoolError);
impl_from_trait!(diesel::result::Error);
impl_from_trait!(actix_web::error::BlockingError);

use actix_web::{HttpResponse, ResponseError};

// エラー時にApiError型からHTTPレスポンスへ変換できるよう
// ApiErrorにRespinseError traitを実装し、エラーごとのHttpResponseを返す様にしている
impl ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ApiError::NotFound => {
                HttpResponse::NotFound().finish()
            }
            ApiError::Other(_) => {
                HttpResponse::ServiceUnavailable()
                    .finish()
            }
        }
    }
}