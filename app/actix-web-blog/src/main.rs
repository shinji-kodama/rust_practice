
mod error;
mod repository;
mod schema;

use actix_web::{web, App, HttpResponse, HttpServer};
// 追加
use actix_web::middleware::{Logger, NormalizePath};
use error::ApiError;
use repository::{NewPost, Repository};

// エラーの可能性があるため、Result型を返すエンドポイント
// 引数はrepoとnew_post
#[actix_web::post("/posts")]
async fn create_post(
    repo: web::Data<Repository>,  // サーバー起動時に①で共有データとして登録されるコレクションプール
    new_post: web::Json<NewPost>, // リクエストボディのJSONをNewPost型にでシリアライズした値
) -> Result<HttpResponse, ApiError> {
    let new_post = new_post.into_inner();
    let post = repo.create_post(new_post).await?; // エラー発生の可能性があり、Result型を返す.
    Ok(HttpResponse::Ok().json(post))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

		// 環境変数からURLを取り出す
    let database_url =
        std::env::var("DATABASE_URL").unwrap();

		// repoを作る
		// web::DataはArcという参照カウントを使ったスマートポインタを拡張した型
    // Repositoryのデータ、つまりコネクションプールを各POSTリクエスト処理で共有できる様になる 
    let repo = web::Data::new(Repository::new(  
        &database_url,
    ));

    HttpServer::new(move || {
        App::new()
            .app_data(repo.clone()) // ① 引数のrepoはここで共有データとして登録され、各エンドポイントの関数で参照できる
            .service(create_post)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}