
pub enum OrderSide {
    Bid,
    Ask
}

pub enum OrderType {
    StopMarket,
    StopLimit,
    Limit,
    Market
}

pub trait OrderRequest {

}


pub trait StopOrder {
    fn stop_price(&self) -> f32;
    fn order_qty(&self) -> i32;

    fn to_cancel(&self) -> Self;
    fn amend_to_zero(&self) -> Self;
    fn as_request(&self) -> dyn OrderRequest;

    fn is_bid(&self) -> bool;
    fn is_ask(&self) -> bool;
}


pub enum Datum {
    OrderBook()
}














pub trait Order {

}

pub trait Margin {

}

pub trait Position {

}

pub trait Trade {

}
