
pub struct Config {
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
