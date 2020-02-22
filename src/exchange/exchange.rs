


pub trait ExchangeWorker {

}



pub trait Exchange {
    // get methods
    fn get_long_position(&self) -> i32;
    fn get_short_position(&self) -> i32;
    fn get_sell_open_qty(&self) -> i32;
    fn get_buy_open_qty(&self) -> i32;
    fn get_trading_leverage(&self) -> f32;
    fn get_available_long(&self) -> i64;
    fn get_available_short(&self) -> i64;
    fn get_trading_value_cnt(&self) -> i64;

    // StopLoss methods
    fn check_stop(&self, stop: impl Order, stop_price: f64) -> bool {

    }
    fn gen_stops_for_side(&self) -> (),
}
