extern crate rayon;

use rayon::prelude::*;
use std::time::SystemTime;
use std::fmt::Debug;

struct BaseEngine {
    // Static config
    exchange: u64,
    symbol: u64,
    index_symbol: u64,
    is_live: u64,
    api_key: u64,
    api_secret: u64,
    model_id: u64,
    model_path: u64,
    trading_fraction: f32,
    apply_fracdiff: bool,
    window_size: i32,
    interval: u32,
    face_value: i32,
    max_drift: i32,
    tick_size:f32,
    leverage: i32,
    scaler_high: i32,
    scaler_low: i32,
    state_buffer_size: i32,
    latent_execution_fraction: f32,

    action_space_type: ActionSpaceType,

}

impl BaseEngine {

    fn new(&self) -> BaseEngine {

    }

    fn get_long_position(&self) -> i32{
        // Bitmex represents the position as a scalar variable
        // whereby the sin is indicative of the direction i.e. Long/short
        // returning the max between the current qty and o returns long position
       cmp::max(self.position.current_qty, 0)
    }

    fn get_short_position(&self) -> i32 {
        // Bitmex represents the position as a scalar variable
        // whereby the sin is indicative of the direction i.e. Long/short
        // returning the min between the current qty and o returns short position
       cmp::min(self.position.short_position, 0)
    }

    fn get_sell_open_qty(&self) -> i64 {
        // Returns total qty of open sell orders from position
        self.position.open_order_sell_qty;
    }

    fn get_buy_open_qty(&self) -> i64 {
        // Returns total qty of buy orders from position
        self.position.open_order_buy_qty;
    }

    fn get_trading_leverage(&self) -> f64 {
        // Returns the available trading leverage with respect to the configured
        // trading fraction.
        self.position.leverage * self.trading_fraction
    }

    fn get_avvailable_long(&self) -> i64 {
        // Returns the total amount that is available to the agent in contracts
        // for use in opening a long position.
        let reserved = if self.position.is_short() {
            self.margin.reserved_margin
        } else {
            0
        };

        let notional_long = self.position.get_notional_long();
        let long_equity = self.convert_to_contracts(
            (self.margin.equity - reserved),
            self.orderbook.best_bid
        );
        let available_long = cmp::max(((long_equity * self.get_trading_leverage()).round() - notional_long), 0);

        available_long;
    }

    fn get_available_short(&self) -> i64 {
        // Returns the total amount that is available to the agent in contracts
        // for use in opening a long position.
        let reserved = if self.position.is_long() {
            self.margin.reserved_margin
        } else {
            0
        };

        // Calculate the notional short as the
        let notional_short = self.position.get_notional_long();
        let short_equity = self.convert_to_contracts(
            (self.margin.equity - reserved),
            self.orderbook.best_ask
        );
        let available_short = cmp::max(((short_equity * self.get_trading_leverage()).round() - notional_short), 0);
        available_short;
    }

    fn get_trading_value_cnt(&self) -> i32{
        // Returns the total available trading value in contracts for the current
        // account.
        let equity_as_cnt = self.convert_to_contracts(self.margin.equity, self.position.mark_price);
        let trading_value_cnt = cmp::max((equity_as_cnt * self.get_trading_leverage()), 0);
        trading_value_cnt;
    }

    // TODO split into smaller testable
    fn get_stops_from_position(&self) -> (Array<Order>, Array<Order>) {
        // Generates a set of stop orders based upon the current position and the
        // resultant liquidation price, given the stop fraction.
        let mut new_orders = Vec::new();
        let mut amend_orders = Vec::new();

        let risk_leway = self.position.liquidation_price - self.position.avg_entry_price;
        let stop_leway = risk_leway * self.stop_fraction;
        let stop_price = (self.position.mark_price + stop_leway).round();

        let stops = self.get_open_stop_orders();

        // Filter all long stop orders i.e. buy
        // orders that are used to close a short
        // position at market price
        let long_stops: Vec<Order> = stops
            .into_iter()
            .filter(|ord| ord.is_buy())
            .collect();

        // Filter all short stop orders i.e. sell
        // orders that are used to close a long
        // position at market price
        let short_stops: Vec<Order> = stops
            .into_iter()
            .filter(|ord| ord.is_sell())
            .collect();

        // TODO dry (make more consice)
        // If the position is long
        if self.position.is_short() {

            // Remove any short stop orders
            if (count(short_stops) > 0){
                for order in short_stops.into_iter() {
                    match order {
                        amend_orders.push()
                    }
                }
            }

            // TODO dry
            if (count(long_stops) == 1) {

            } else if (count(short_stops) > 1){

            } else (


            )



        } else if self.position.is_long() {

        }
    }



}
