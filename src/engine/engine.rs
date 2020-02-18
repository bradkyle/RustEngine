extern crate ndarray;

use std::time::SystemTime;
use std::fmt::Debug;
use ndarray::Array1;
use std::cmp;
use

#[derive(Debug)]
pub enum ActionSpaceType {
    Discrete {
        num_actions: i32,
        num_options: i32
    },

    Box {
        num_actions: i32,
        action_high: i32,
        action_low: i32,
    },

}

// Implements agent functionality
// for the engine, inference etc.
trait Core {
    fn exec_action(&self, obs: Array1<f64>);
    fn init_state(&self);
    fn buffer_ready(&self);
    fn process_order_update(&self);
    fn process_position_update(&self);
    fn process_instrument_update(&self);
    fn process_book_udpate(&self);
    fn process_margin_update(&self);
    fn run_ws(&self);
    fn run_agent(&self);
    fn run(&self);
}

trait RestClient {
    fn amend_bulk_orders(&self);
    fn place_bulk_orders(&self);
    fn get_open_limit_orders(&self);
    fn get_open_stop_orders(&self);
}

trait Engine {
    fn get_action(&self);
    fn gen_orders_from_action(&self);
}

// Implements Action and ingress functionality
// for
impl<T> Core for T where T: Engine {
    fn exec_action(&self, obs: Array1<f64>){
       // TODO
       println!("The observation is {}", obs)
    }

    fn get_long_position(&self) -> i32 {
        self.long_position
    }
}

// Composition over inheritance
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

    position: Position,
    margin: Margin,
    orderbook: Orderbook;
    instrument: Instrument,
    orders: Array<Orders>
}

impl BaseEngine {
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
        let available_short = cmp::max(((long_equity * self.get_trading_leverage()).round() - notional_long), 0);
        available_short;
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

}


pub struct DiscreteEngne {
    config: EngineConfig,
}

pub struct BoxEngine {
    config: EngineConfig,
}
