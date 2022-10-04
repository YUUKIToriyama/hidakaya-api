use rusqlite::{params, Connection};

use crate::model::category::Category;

pub fn get_all(connection: &Connection) -> Vec<Category> {
    let mut statement = connection.prepare("SELECT id, name FROM category").unwrap();
    let rows = statement
        .query_map(params![], |row| {
            Ok(Category {
                id: row.get(0).unwrap(),
                name: row.get(1).unwrap(),
            })
        })
        .unwrap();
    let mut result: Vec<Category> = Vec::new();
    for row in rows {
        result.push(row.unwrap());
    }
    result
}
