use serde::{Deserialize, Serialize};

use crate::api::PaginationResponse;
use crate::client::Client;
use crate::error::Error;
use crate::query_tuple;

#[derive(Serialize, Deserialize, Debug)]
pub struct Statement {
    pub id: Option<String>,
    pub order_id: Option<String>,
    pub external_id: Option<String>,
    #[serde(rename = "type")]
    pub statement_type: String,
    pub quantity: i64,
    pub isin: String,
    pub isin_title: String,
    pub date: String,
    pub created_at: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum StatementType {
    OrderBuy,
    OrderSell,
    Split,
    Import,
    Snx,
}
type StatementPagination = PaginationResponse<Statement>;

impl Client {
    /// Get all change events happening to your positions.
    pub fn get_statements(
        &self,
        limit: Option<i64>,
        page: Option<u32>,
    ) -> Result<StatementPagination, Error> {
        const PATH: &str = "positions/statements";

        let mut query: Vec<String> = vec![];
        Client::get_query_string(query_tuple!(limit), &mut query);
        Client::get_query_string(query_tuple!(page), &mut query);

        let resp = self.get_with_query::<StatementPagination, Vec<String>>(PATH, query);
        match resp {
            Ok(r) => Ok(r),
            Err(e) => Err(e),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::env;

    use crate::client;

    #[test]
    fn test_get_statement() {
        dotenv::dotenv().unwrap();
        let api_key = env::var("LEMON_MARKET_TRADING_API_KEY").unwrap();
        let client = client::Client::paper_client(&api_key);
        let page = 1;
        let statements = client.get_statements(None, None).unwrap();
        assert_eq!(statements.status, "ok");
    }
}
