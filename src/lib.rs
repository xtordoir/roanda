use std::fmt::Debug;
use chrono::DateTime;
use serde::Deserialize;
use serde::Serialize;
use serde::de::DeserializeOwned;

#[derive(Debug)]
pub struct Tick {
    pub time: i64,
    pub bid: f64,
    pub ask: f64,
}

impl Tick {

    pub fn time(&self) -> i64 {
        self.time / 1000
    }
    pub fn price(&self) -> f64 {
        (self.bid+self.ask)/2.0
    }

    pub fn buy_price(&self) -> f64 {
        self.ask
    }
    pub fn sell_price(&self) -> f64 {
        self.bid
    }

    pub fn spread(&self) -> f64 {
        self.bid - self.ask
    }
}

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
    pub quote_home_conversion_factors: Option<QuoteHomeConversionFactors>,
    pub status: String,
    pub time: String,
    pub units_available: Option<UnitsAvailable>,
}

impl Pricing {
    pub fn get_tick(&self) -> Tick {
        Tick{
            time:  DateTime::parse_from_rfc3339(self.prices.first().map(|p| p.time.clone()).unwrap().as_str()).unwrap().timestamp(),
            bid: (self.prices.first().map(|p| p.bids.first().map(|l| l.price.clone()).unwrap()).unwrap()).parse::<f64>().unwrap(),
            ask: (self.prices.first().map(|p| p.asks.first().map(|l| l.price.clone()).unwrap()).unwrap()).parse::<f64>().unwrap(),
        }
    }
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
