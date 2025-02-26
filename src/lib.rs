use std::fmt::Debug;
use chrono::DateTime;

use std::collections::HashMap;

use serde::Deserialize;
use serde::Serialize;
use serde::de::DeserializeOwned;

//use serde_json::json;

// Instruments related definitions

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Instruments {
    pub instruments: Vec<Instrument>,
    #[serde(rename = "lastTransactionID")]
    pub last_transaction_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Instrument {
    pub display_name: String,
    pub display_precision: i64,
    pub margin_rate: String,
    pub maximum_order_units: String,
    pub maximum_position_size: String,
    pub maximum_trailing_stop_distance: String,
    pub minimum_trade_size: String,
    pub minimum_trailing_stop_distance: String,
    pub name: String,
    pub pip_location: i64,
    pub trade_units_precision: i64,
    #[serde(rename = "type")]
    pub type_field: String,
}

// Utility Tick struct ( not part of oanda API definitions )

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

// Pricing definitions

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
    /**
     * convert a Pricing response into a Tick
     */
    pub fn get_tick(&self) -> Tick {
        Tick{
            time:  DateTime::parse_from_rfc3339(self.prices.first().map(|p| p.time.clone()).unwrap().as_str()).unwrap().timestamp(),
            bid: (self.prices.first().map(|p| p.bids.first().map(|l| l.price.clone()).unwrap()).unwrap()).parse::<f64>().unwrap(),
            ask: (self.prices.first().map(|p| p.asks.first().map(|l| l.price.clone()).unwrap()).unwrap()).parse::<f64>().unwrap(),
        }
    }
    pub fn is_tradeable(&self) -> bool {
        self.prices.first().map(|p| p.status.clone()).unwrap() == "tradeable"
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

// Positions Definitions

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OnePosition {
    #[serde(rename = "lastTransactionID")]
    pub last_transaction_id: String,
    pub position: Position,
}

impl OnePosition {
    pub fn new(instrument: String) -> Self {
        Self{
            last_transaction_id: "".to_string(),
            position: Position::empty(instrument),
        }

    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Positions {
    #[serde(rename = "lastTransactionID")]
    pub last_transaction_id: String,
    pub positions: Vec<Position>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Position {
    pub instrument: String,
    pub long: Long,
    pub pl: String,
    #[serde(rename = "resettablePL")]
    pub resettable_pl: String,
    pub short: Short,
    #[serde(rename = "unrealizedPL")]
    pub unrealized_pl: String,
}
impl Position {
    pub fn empty(instrument: String) -> Self {
        Self{
            instrument: instrument,
            long: Long::empty(),
            short: Short::empty(),
            pl: "0".to_string(),
            resettable_pl: "0".to_string(),
            unrealized_pl: "0".to_string(),
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Long {
    pub average_price: Option<String>,
    pub pl: String,
    #[serde(rename = "resettablePL")]
    pub resettable_pl: String,
    #[serde(rename = "tradeIDs")]
    pub trade_ids: Option<Vec<String>>,
    pub units: String,
    #[serde(rename = "unrealizedPL")]
    pub unrealized_pl: String,
}
impl Long {
    pub fn empty() -> Self {
        Self { 
            average_price: None, 
            pl: "0".to_string(),
            resettable_pl: "0".to_string(), 
            trade_ids: None, 
            units: "0".to_string(), 
            unrealized_pl: "0".to_string() 
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Short {
    pub average_price: Option<String>,
    pub pl: String,
    #[serde(rename = "resettablePL")]
    pub resettable_pl: String,
    #[serde(rename = "tradeIDs")]
    pub trade_ids: Option<Vec<String>>,
    pub units: String,
    #[serde(rename = "unrealizedPL")]
    pub unrealized_pl: String,
}
impl Short {
    pub fn empty() -> Self {
        Self { 
            average_price: None, 
            pl: "0".to_string(),
             resettable_pl: "0".to_string(), 
            trade_ids: None, 
            units: "0".to_string(), 
            unrealized_pl: "0".to_string() 
        }
    }
}

// Order definitions
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderRequest {
    pub order: Order,
}
impl OrderRequest {
    pub fn market(units: String, instrument: String) -> Self {
        Self {
            order: Order {
                units: units,
                instrument: instrument,
                time_in_force: "FOK".to_owned(),
                type_field: "MARKET".to_owned(),
                position_fill: "DEFAULT".to_owned(),
            }
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Order {
    pub units: String,
    pub instrument: String,
    pub time_in_force: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub position_fill: String,
}




impl OrderRequest {
    pub fn new_market(units: String, instrument: String) -> Self {
        Self {
            order: Order {
                units: units,
                instrument: instrument,
                time_in_force: "FOK".to_owned(),
                type_field: "MARKET".to_owned(),
                position_fill: "DEFAULT".to_owned(),
            }
        }
    }

    pub fn to_string(&self) -> String {
        return serde_json::ser::to_string(self).ok().unwrap();
    }
}



#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostOrderResponse {
    #[serde(rename = "lastTransactionID")]
    pub last_transaction_id: String,
    pub order_create_transaction: Option<OrderCreateTransaction>,
    pub order_fill_transaction: Option<OrderFillTransaction>,
    #[serde(rename = "relatedTransactionIDs")]
    pub related_transaction_ids: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderCreateTransaction {
    #[serde(rename = "accountID")]
    pub account_id: String,
    #[serde(rename = "batchID")]
    pub batch_id: String,
    pub id: String,
    pub instrument: String,
    pub position_fill: String,
    pub reason: String,
    pub time: String,
    pub time_in_force: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub units: String,
    #[serde(rename = "userID")]
    pub user_id: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderFillTransaction {
    pub account_balance: String,
    #[serde(rename = "accountID")]
    pub account_id: String,
    #[serde(rename = "batchID")]
    pub batch_id: String,
    pub financing: String,
    pub id: String,
    pub instrument: String,
    #[serde(rename = "orderID")]
    pub order_id: String,
    pub pl: String,
    pub price: String,
    pub reason: String,
    pub time: String,
    pub trade_opened: Option<TradeOpened>,
    #[serde(rename = "type")]
    pub type_field: String,
    pub units: String,
    #[serde(rename = "userID")]
    pub user_id: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TradeOpened {
    #[serde(rename = "tradeID")]
    pub trade_id: String,
    pub units: String,
}

pub struct Client {
    token: String,
    url: String,
    account: String,
    client: reqwest::Client,
    pub instruments: HashMap<String, Instrument>,
}

impl Client {
    pub fn new(url: String, account: String, token: String) -> Client {
        let ret = Client {
            token: token,
            account: account,
            url: url,
            client: reqwest::Client::new(),
            instruments: HashMap::new(),
        };
        ret
    }

    /**
     * calling get_instruments retuns the Instruments and set them in the client 
     */
    pub async fn get_instruments(&mut self) -> Option<Instruments> {
        let request_url = format!("{}/v3/accounts/{}/instruments",self.url.clone(), self.account);
        let instruments_opt = self.get::<Instruments>(request_url).await;
        instruments_opt.iter().for_each(|i| {
            i.instruments.iter().for_each(|instrument| {
                let i_clone = instrument.clone();
                self.instruments.insert(i_clone.name.clone(), i_clone);
            });
        });
        return instruments_opt;
    }

    pub async fn get_instruments_from(&mut self, instruments: Vec<String>) -> Option<Instruments> {
        let query = instruments.join(",");
        let request_url = format!("{}/v3/accounts/{}/instruments?{}",self.url.clone(), self.account, query);
        let instruments_opt = self.get::<Instruments>(request_url).await;
        instruments_opt.iter().for_each(|i| {
            i.instruments.iter().for_each(|instrument| {
                let i_clone = instrument.clone();
                self.instruments.insert(i_clone.name.clone(), i_clone);
            });
        });
        return instruments_opt;
    }

    pub async fn get_pricing(&self, instrument: String) -> Option<Pricing> {
        let request_url = format!("{}/v3/accounts/{}/pricing?instruments={}",self.url.clone(), self.account, instrument);
        return self.get::<Pricing>(request_url).await;
    }

    pub async fn get_position(&self, instrument: String) -> Option<OnePosition> {
        let request_url = format!("{}/v3/accounts/{}/positions/{}",self.url.clone(), self.account, instrument);
        return self.get::<OnePosition>(request_url).await;
    }

    pub async fn get_open_positions(&self) -> Option<Positions> {
        let request_url = format!("{}/v3/accounts/{}/openPositions",self.url.clone(), self.account);
        return self.get::<Positions>(request_url).await;
    }


    pub async fn post_order_request(&self, order: OrderRequest) -> Option<PostOrderResponse> {
        let request_url = format!("{}/v3/accounts/{}/orders",self.url.clone(), self.account);
        return self.post::<OrderRequest, PostOrderResponse>(request_url, order).await;
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

        /**
     * private method to POST on a resource and parse response as T
     */
    async fn post<R, T>(&self, request_url: String, request: R) -> Option<T> where
    T: DeserializeOwned + Debug, R: Serialize {
        let response: Result<reqwest::Response, reqwest::Error> = self.client
            .post(request_url)
            .bearer_auth(self.token.clone())
            .json(&request)
            .send()
            .await;

        if let Some(res) = response.ok() {
            //let xxx = res.bytes().await.ok().unwrap();
            let result = res.json::<T>().await;
            if result.is_err() {
                println!("{:#?}", result.err().unwrap());
                return None;
            }
            //println!("{:#?}", xxx);
            return result.ok();
        } else {
            eprint!("http error..");
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
