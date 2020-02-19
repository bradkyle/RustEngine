#![feature(box_patterns)]

extern crate rkdb;
use std::error::Error;
use std::fmt;

use rkdb::{
    api,
    kbindings::*
};

struct StateEngine {
    handle: api::Handle,
}

impl StateEngine {

    fn new(&self) -> StateEngine {

    }

    // Handle State udpates
    fn process_order_udpate(&self, order:Order) {

    }

    fn process_instrument_update(&self, instrument:Instrument) {

    }

    fn process_orderbook_update(&self, depth:Depth) {

    }

    fn process_margin_update(&self, margin:Margin) {

    }

    fn process_position_update(&self, position:Position) {

    }

    fn process_tradebin_update(&self, tradebin:TradeBin) {

    }

    fn process_trade_update(&self, trade:Trade) {

    }

    // Serve state in grpc format

    // Convert state into observations

    fn run(&self) {

    }
}


struct StateEngineClient {

}

impl StateEngineCLient {

}
