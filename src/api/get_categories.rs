use actix_web::{get, web::Json};

use crate::model::category::Category;
use crate::repository::get_all;
use crate::repository::open_db;

#[get("/categories")]
pub async fn get_categories() -> Json<Vec<Category>> {
    // DBの接続
    let connection = open_db::open_db().unwrap();
    let result = get_all::get_all(&connection);
    Json(result)
}
