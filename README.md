# Oanda client in rust

Partial implementation of the Oanda API in rust

## Implemented endpoints

### Instruments

- get_instruments(&self)
- get_instruments_from(&mut self, instruments: Vec<String>)

### Pricing

- get_pricing(&self, instrument: String)

### Positions

- get_position(&self, instrument: String)
- get_open_positions(&self)


## Also required to run trading strategies with market orders

- post_order(&self, order: &OrderRequest)

## Required to run multiple instruments strategies concurrently

- get_pricing(&self, instruments: Vec<String>)

## Required for strategies using Limit Orders

TODO