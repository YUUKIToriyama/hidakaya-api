use actix_web::{get, web::Json};

use crate::model::category::Category;

#[get("/categories")]
pub async fn get_categories() -> Json<Vec<Category>> {
    return Json(vec![
        Category {
            id: 1,
            name: "ラーメン".to_string(),
        },
        Category {
            id: 2,
            name: "定食".to_string(),
        },
    ]);
}
