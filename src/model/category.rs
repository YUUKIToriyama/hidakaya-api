use serde::Serialize;

#[derive(Serialize)]
pub struct Category {
    /** カテゴリID */
    pub(crate) id: u8,
    /** カテゴリ名 */
    pub(crate) name: String,
}
