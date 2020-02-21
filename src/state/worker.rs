
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

    fn connect(&self) -> {
        let bm = BitMEX::with_credential(&var("BITMEX_KEY")?, &var("BITMEX_SECRET")?);
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

    fn process_trade_update(&self, trade:Trade) {

    }

    // Serve state in grpc format

    // Convert state into observations

    fn run(&self) {
        let mut client = bm.websocket().await?;

        let expires = (Utc::now() + Duration::seconds(30)).timestamp();

        client
            .send(Command::authenticate(&bm, expires).unwrap())
            .await?;

        client
            .await?;

        client
            .send(Command::Subscribe(vec![
                Topic::Margin,
                Topic::Position,
                Topic::Trade(Some("XBTUSD".to_string())),
                Topic::OrderBook10(Some("XBTUSD".to_string())),
                Topic::Po
            ]))
            .await?;

        while let Some(msg) = client.next().await {
            println!("{:?}", msg);
        }
        Ok(())

    }
}
