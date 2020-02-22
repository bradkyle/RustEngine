

trait Store {
    fn get_order(&self);
    fn get_order_by_client_oid(&self);
    fn add_order(&self);
    fn update_order(&self);

    fn get_position(&self);
    fn update_position(&self);

    fn get_margin(&self);
    fn update_margin(&self);

    fn get_orderbook(&self);
    fn update_orderbook(&self);

    fn get_trade(&self);
    fn add_trade(&self);

    fn get_instrument(&self);
    fn update_instrument(&self);

    fn get_observation_vector(&self);
}
