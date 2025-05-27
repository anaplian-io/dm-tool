use crate::utilities::Die;
use axum::Json;
use enum_iterator::all;

pub async fn list_dice() -> Json<Vec<Die>> {
    Json(all::<Die>().collect())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utilities::Die::{D4, D6, D8, D10, D12, D20, Raw};

    #[tokio::test]
    async fn test_list_dice() {
        assert_eq!(list_dice().await.0, vec![D4, D6, D8, D10, D12, D20, Raw])
    }
}
