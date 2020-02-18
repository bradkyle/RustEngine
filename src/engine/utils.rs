
// fn to_cnt(&self, mrg_amt: f64, price: f64) -> i64;
// fn to_mrg(&self, cnt_amt: i64, price: f64) -> f64;


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
