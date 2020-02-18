extern crate ndarray;

use std::time::SystemTime;
use std::fmt::Debug;
use ndarray::Array1;

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
    fn get_long_position(&self) -> i32;
    fn get_short_position(&self) -> i32;
    fn get_sell_open_qty(&self) -> i32;
    fn get_buy_open_qty(&self) -> i32;
    fn get_avvailable_long(&self) -> i32;
    fn get_available_short(&self) -> i32;
    fn get_trading_value_cnt(&self) -> i32;
    fn get_funding_rate(&self) -> f32;
    fn get_account_se(&self) -> Array1<f64>;
    fn process_order_update(&self);
    fn process_position_update(&self);
    fn process_instrument_update(&self);
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
}


pub struct EngineConfig {
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



pub struct DiscreteEngne {
    config: EngineConfig,
}

pub struct BoxEngine {
    config: EngineConfig,
}
