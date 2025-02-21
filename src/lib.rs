use std::fmt::Debug;

use serde::Deserialize;
use serde::Serialize;
use serde::de::DeserializeOwned;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pricing {
    pub prices: Vec<Price>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Price {
    pub asks: Vec<Ask>,
    pub bids: Vec<Bid>,
    pub closeout_ask: String,
    pub closeout_bid: String,
    pub instrument: String,
    pub quote_home_conversion_factors: QuoteHomeConversionFactors,
    pub status: String,
    pub time: String,
    //pub units_available: UnitsAvailable,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ask {
    pub liquidity: i64,
    pub price: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Bid {
    pub liquidity: i64,
    pub price: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuoteHomeConversionFactors {
    pub negative_units: String,
    pub positive_units: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UnitsAvailable {
    pub default: Default,
    pub open_only: OpenOnly,
    pub reduce_first: ReduceFirst,
    pub reduce_only: ReduceOnly,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Default {
    pub long: String,
    pub short: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenOnly {
    pub long: String,
    pub short: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReduceFirst {
    pub long: String,
    pub short: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReduceOnly {
    pub long: String,
    pub short: String,
}

pub struct Client {
    token: String,
    url: String,
    account: String,
    client: reqwest::Client,
}

impl Client {
    pub fn new(url: String, account: String, token: String) -> Client {
        let ret = Client {
            token: token,
            account: account,
            url: url,
            client: reqwest::Client::new(),
        };
        ret
    }

    pub async fn get_pricing(&self, instrument: String) -> Option<Pricing> {
        let request_url = format!("{}/v3/accounts/{}/pricing?instruments={}",self.url.clone(), self.account, instrument);
        return self.get::<Pricing>(request_url).await;
    }

    /**
     * private method to GET a resource and parse response as T
     */
    async fn get<T>(&self, request_url: String) -> Option<T> where
    T: DeserializeOwned + Debug {
        let response = self.client
        .get(request_url)
        .bearer_auth(self.token.clone())
        .send()
        .await;
       
        if let Some(res) = response.ok() {
            return res.json().await.ok();
        }
        
        None
    }

}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
