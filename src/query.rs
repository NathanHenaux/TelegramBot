use serde::{Serialize, Deserialize};
use graphql_client::GraphQLQuery;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/schema.graphql",
    query_path = "src/graphql/query.graphql",
    response_derives = "Debug"
)]
struct AccurateCounters;

type Size = u32;

// fn main() {
//     let variables = accurate_counters::Variables {
//         query_sales: "".to_string(),
//         query_rentals: "".to_string(),
//     };

//     let request_body = AccurateCountersQuery::build_query(variables);

//     // Ensuite, vous pouvez envoyer la requête à votre serveur GraphQL comme vous le souhaitez
// }


#[derive(Debug)]
pub struct Counters{
    pub sales: u32,
    pub rentals: u32
}

pub async fn get_accurate_counters() -> anyhow::Result<Counters>{
    todo!()
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




// https://github.com/graphql-rust/graphql-client/blob/main/examples/web/src/puppy_smiles.graphql

// query PuppySmiles($after: String) {
//     reddit {
//       subreddit(name: "puppysmiles") {
//         newListings(limit: 6, after: $after) {
//           title
//           fullnameId
//           url
//         }
//       }
//     }
//   }