# Oanda client in rust

Partial implementation of the Oanda API in rust

## Implemented endpoints

- get_instruments(&self)
- get_pricing(&self, instrument: String)

## Also required to run trading strategies with market orders

- get_position(&self, instrument: String)
- post_order_request(&self, order: &OrderRequest)

## Required to run multiple instruments strategies concurrently

- get_pricing(&self, instruments: Vec<String>)
- get_open_positions(&self)

## Required for strategies using Limit Orders

TODO