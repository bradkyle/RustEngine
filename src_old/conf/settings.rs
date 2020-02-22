use std::env;
use config::{ConfigError, Config, File, Environment};
use serde_derive::{Deserialize};

// Determines the action space that
// the agent uses.
#[derive(Debug, Deserialize)]
pub enum ActionSpaceType {
    Discrete,
    DualBox,
    SimpleBox
}

// Long term actions are for agents
// that have a step rate of 1 minute
// or above, the output of which is
// generally used by the short term
// agent
#[derive(Debug, Deserialize)]
pub enum ActionTypes {
    Hold,
    FlattenLimit,
    FracLongLimit,
    FracShortLimit,
    ExecShortMarket,
    ExecLongMarket,
    FlattenMarket,
    FullLongLimit,
    FullShortLimit,
    IncLeverage,
    DecLeverage,
    DualLimit(i32, i32),
    SimpleBox(f32),
    DualBox(f32, f32)
}

fn get_default_action_set() ->  {

}

// Global multi environment configuration
// for the
#[derive(Debug, Deserialize)]
pub struct Settings {
    // Static config
    exchange: String,
    symbol: String,
    index_symbol: String,
    is_live: bool,
    api_key: String,
    api_secret: String,
    model_id: String,
    model_path: String,
    trading_fraction: f32,
    apply_fracdiff: bool,
    window_size: i32,
    interval: i32,
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

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let mut s = Config::new();

        // Start off by merging in the "default" configuration file
        s.merge(File::with_name("config/default"))?;

        // Add in the current environment file
        // Default to 'development' env
        // Note that this file is _optional_
        let env = env::var("RUN_MODE").unwrap_or("development".into());
        s.merge(File::with_name(&format!("config/{}", env)).required(false))?;

        // Add in a local configuration file
        // This file shouldn't be checked in to git
        s.merge(File::with_name("config/local").required(false))?;

        // Add in settings from the environment (with a prefix of APP)
        // Eg.. `APP_DEBUG=1 ./target/app` would set the `debug` key
        s.merge(Environment::with_prefix("app"))?;

        // Now that we're done, let's access our configuration
        println!("debug: {:?}", s.get_bool("debug"));
        println!("database: {:?}", s.get::<String>("database.url"));

        // You can deserialize (and thus freeze) the entire configuration as
        s.try_into()
    }


}
