use crate::schema::*;
use diesel::r2d2::{self, ConnectionManager};
use diesel::sqlite::SqliteConnection;
use diesel::prelude::*;
use crate::error::ApiError;
use serde::{Deserialize, Serialize};
use actix_web::web;

type DbPool =
    r2d2::Pool<ConnectionManager<SqliteConnection>>;

// POSTリクエスト時に受け取る記事データを持つ型を定義
#[derive(Deserialize, Insertable)] // Insertable: DieselがNewPost型からDBへINSERTできるようになる
#[diesel(table_name = posts)] // 対象テーブルの指定
pub struct NewPost {
    title: String,
    body: String,
}

// レコードの全フィールドを持つ型
#[derive(Serialize, Queryable)] // Queryable: DieselがDBのレコードを格納できるようにする
pub struct Post {
    id: i32,
    title: String,
    body: String,
    published: bool,
}

pub struct Repository {
  pool: DbPool,
}

impl Repository {
  pub fn new(database_url: &str) -> Self {
    let manager = ConnectionManager::<
        SqliteConnection,
    >::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create a pool.");  // エラーの場合はパニックさせて強制終了
    Self { pool }
  }


    // create_postメソッドを追加
    // NewPost型の値を引数に取り、DBにINSERT
    // 成功時：登録したレコード(Post型)を返す
  // エラー時は?が書かれているタイミングでApiError型が返る
  pub async fn create_post(
    &self,
    new_post: NewPost,
  ) -> Result<Post, ApiError> {
    let mut conn = self.pool.get()?;
    let post = web::block(move || {
        diesel::insert_into(posts::table)
            .values(new_post)
            .get_result(&mut conn)
    })
    .await??; // この??は?を２回適用している

    Ok(post)
  }


  pub async fn list_posts(
    &self,
  ) -> Result<Vec<Post>, ApiError> {
    let mut conn = self.pool.get()?;
    let res = web::block(move || {
        posts::table.load(&mut conn)
    })
    .await??;

    Ok(res)
  }

  pub async fn get_post(
      &self,
      id: i32,
  ) -> Result<Post, ApiError> {
      let mut conn = self.pool.get()?;
      let res = web::block(move || {
          posts::table
              .find(id)
              .first(&mut conn)
              .optional()
      })
      .await??
      .ok_or(ApiError::NotFound)?;

      Ok(res)
  }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_conn() {
        let database_url =
            std::env::var("DATABASE_URL").unwrap();
        let repo = Repository::new(&database_url);
        assert!(repo.pool.get().is_ok());
    }
}