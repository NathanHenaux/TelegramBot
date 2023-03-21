use juniper::Variables;
use reqwest;
use serde::{Serialize, Deserialize};
use graphql_client::{GraphQLQuery, Response};

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/schema.graphql",
    query_path = "src/graphql/query.graphql",
    response_derives = "Debug"
)]
struct AccurateCountersQuery;

type Size = u32;

pub async fn get_accurate_counters() -> anyhow::Result<Counters>{   
  
  let variables = accurate_counters_query::Variables {
    sales_query: "".to_string(),
    rentals_query: "".to_string(),
  };
  let  request_body = AccurateCountersQuery::build_query(variables);

  let client = reqwest::Client::new();
  let mut res = client.post("https://wymmo.com/graphql").json(&request_body).send().await?;
  let response_body: Response<accurate_counters_query::ResponseData> = res.json().await?;

  let data = response_body.data.unwrap();

  Ok(Counters{ rentals : data.accurate_counters.rentals, sales: data.accurate_counters.sales })
}

/// This is only a return type !
/// This is not something to make the graphql query
#[derive(Debug)]
pub struct Counters{
    pub sales: u32,
    pub rentals: u32
}


#[cfg(test)]
mod tests {
  use super::*;

  #[tokio::test]
  async fn test_get_accurate_counters() -> anyhow::Result<()> {
    
    let counters = get_accurate_counters().await?;
    dbg!(&counters);
    // Si ça n'a pas planté, c'est que ça a du bien se passer !
    // et on check que c'est pas 0
    assert_ne!(counters.sales, 0);
    assert_ne!(counters.rentals, 0);

    Ok(())
  }
}

