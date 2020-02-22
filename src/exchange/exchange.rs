
use super::models::{StopOrder, OrderRequest, OrderSide, OrderType};


pub trait ExchangeWorker {

    // Functionality for updating the state
    // of the exchange client.
    fn run_state_engine(&self);
    fn process_ws_update(&self);
}


// Assumes only one position is open
pub trait ExchangeClient {

    fn is_short(&self) -> bool;
    fn is_long(&self) -> bool;

    // get methods
    fn current_position(&self) -> i32;
    fn opposite_position(&self) -> i32;
    fn long_position(&self) -> i32;
    fn short_position(&self) -> i32;
    fn sell_open_qty(&self) -> i32;
    fn buy_open_qty(&self) -> i32;
    fn trading_leverage(&self) -> f32;
    fn available_long(&self) -> i64;
    fn available_short(&self) -> i64;
    fn trading_value_cnt(&self) -> i64;
    fn liquidation_price(&self) -> f32;
    fn average_entry_price(&self) -> f32;
    fn mark_price(&self) -> f32;

    fn get_open_stop_orders(&self) -> Vec<dyn StopOrder>;

    // Check if the given stop loss order adequately matches the given
    // stop price and is equal to the opposite of the current position.
    fn check_stop<T: StopOrder>(&self, stop: T, stop_price: f32) -> bool {
        if stop.stop_price() != stop_price
            || stop.order_qty() != self.opposite_position() {
            true
         } else {
            false
         }
    }

    // Recieves a vector of stop orders for a given side of the
    // order book and subsequently amends their quantity to
    // zero which effectively cancels them.
    fn stop_amend_zero_for_side<T: StopOrder, U: OrderRequest>(
        &self,
        stops: Vec<T>,
        amend_orders: &mut Vec<U>
    ) {
        if stops.len()>0{
            for stop in stops.into_iter() {
                let amend_request: OrderRequest = stop.to_cancel().as_request();
                amend_orders.push(amend_request);
            }
        }
    }

    // Anneals the current stop orders for a given position to
    // a singular stop order of the opposing sin and equal
    // quantity to the current position
    fn stop_anneal_qty_for_side<T: StopOrder, U: OrderRequest>(
        &self,
        stops: Vec<T>,
        amend_orders: &mut Vec<U>,
        new_orders: &mut Vec<U>,
        side: OrderSide
    ) {
        if stops.len() == 1 {
            println!("not implemented");
        } else if stops.len() > 1 {
            println!("not implemented");
        } else {
            println!("not implemented");
        }
    }

    // Generates a set of order requests that serve to anneal the current stop
    // quantity to a desired stop quantity where the current position is effectively
    // opposed.
    fn gen_stops_for_side<T: StopOrder, U:OrderRequest>(
        &self,
        side_orders: Vec<T>,
        opps_orders: Vec<T>,
        side: OrderSide,
        amend_orders: &mut Vec<U>,
        new_orders: &mut Vec<U>
    ) {
        self.stop_amend_zero_for_side(
            opps_orders,
            amend_orders
        );

        self.stop_anneal_qty_for_side(
            side_orders,
            amend_orders,
            new_orders,
            side
        )
    }

    fn generate_stops_from_position<T: StopOrder, U:OrderRequest>(
        &self,
        stop_fraction: f32,

    ) -> (Vec<U>, Vec<U>) {
        let mut new_orders = Vec::new();
        let mut amend_orders = Vec::new();

        let risk_leway: f32 = self.liquidation_price() - self.average_entry_price();
        let stop_leway: f32 = risk_leway * stop_fraction;
        let stop_price: f32 = self.mark_price() + stop_leway;

        let stops: Vec<dyn StopOrder> = self.get_open_stop_orders();

        // Filter all long stop orders i.e. buy
        // orders that are used to close a short
        // position at market price
        let long_stops: Vec<dyn StopOrder> = stops
            .into_iter()
            .filter(|ord| ord.is_bid())
            .collect();

        // Filter all short stop orders i.e. sell
        // orders that are used to close a long
        // position at market price
        let short_stops: Vec<dyn StopOrder> = stops
            .into_iter()
            .filter(|ord| ord.is_ask())
            .collect();

        if self.is_short() {
            self.gen_stops_for_side(
                short_stops,
                long_stops,
                OrderSide::Ask,
                amend_orders,
                new_orders
            )
        } else if self.is_long() {
            self.gen_stops_for_side(
                long_stops,
                short_stops,
                OrderSide::Bid,
                amend_orders,
                new_orders
            )
        }

        // TODO return actual value
        (new_orders, amend_orders)
    }

    // Generates a set of orders to be placed
    // in the orderbook gven a set of buy side deltas
    // indicative of the changes in order quantities
    // that need to occur per level of the orderbook.
    // with ascending divergence from the spread
    // i.e. bid_deltas[990, 989, 987, ...]
    //      ask_deltas[991, 992, 993, ...]
    fn generate_orders_from_deltas(
        &self,
        bid_deltas: Vec<f32>,
        ask_deltas: Vec<f32>,

    )
}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(1+2, 3);
    }

}
