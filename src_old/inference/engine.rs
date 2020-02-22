extern crate rayon;

use rayon::prelude::*;
use std::time::SystemTime;
use std::fmt::Debug;

struct InferenceEngine {
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
    state_engine_client: StateEngineClient,
    agent_inference_client: AgentInferenceClient

}

impl InferenceEngine {

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

    fn check_stop(&self, stop: Order, stop_price: f64) -> bool {
        // Checks if the current stop is not correctly set i.e. another
        // stop order needs to be placed.
        if (long_stop.stop_px != stop_price
            || long_stop.order_qty != -self.position.current_qty) {
            true;
        }
    }

    // Appends orders to given new_orders/amend_orders arrays
    // depending on the side given to the function.
    fn gen_stops_for_side() -> (Array<Order>, Array<Order>) {
            // Remove any short stop orders
            if (count(short_stops) > 0){
                for order in short_stops.into_iter() {
                    let amend_order = Order{
                        order_id: order.order_id,
                        order_qty: 0,
                        ..order
                    }
                    amend_orders.push(amend_order)
                }
            }

            if (count(long_stops) == 1) {

                // get the first/only long stop
                let long_stop = long_stops[0];

                // if the stop loss price is not equal to the stop price
                // defined above or the stop loss quantity is not equal
                // to the inverse of the position size amend stop loss order.
                if check_stop(long_stop, stop_price) {
                    let amend_order = Order{
                        stop_px: stop_price,
                        order_qty: -self.position.current_qty,
                        ..order
                    }
                    amend_orders.push(amend_order);
                }

            } else if (count(short_stops) > 1){

                // group by stop price, if there is one order at the
                // current stop price then remove others and amend this
                // order else remove all others and place new stop order

            } else {
                // If there are no stop orders in existence then generate
                // a new stop order opposing the current position.
                let order = Order{
                    ord_type:OrdType::Stop,
                    order_qty: -self.position.current_qty,
                    stop_px: stop_price,
                    side:
                    symbol: self.symbol
                }
                new_orders.push(new_order);
            }

    }

    // TODO split into smaller testable
    fn get_stops_from_position(&self) -> (Array<Order>, Array<Order>) {
        // Generates a set of stop orders based upon the current position and the
        // resultant liquidation price, given the stop fraction.
        let mut new_orders = Vec::new();
        let mut amend_orders = Vec::new();

        let risk_leway: f64 = self.position.liquidation_price - self.position.avg_entry_price;
        let stop_leway: f64 = risk_leway * self.stop_fraction;
        let stop_price: f64 = self.position.mark_price + stop_leway;

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

        // TODO dry for long/short (make more consice)
        // If the position is long
        if self.position.is_short() {
            new_orders, amend_orders = self.gen_stops_for_side()
        } else if self.position.is_long() {
            new_orders, amend_orders = self.gen_stops_for_side()
        }

        (new_orders, amend_orders)
    }





}
