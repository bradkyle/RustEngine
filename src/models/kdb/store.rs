extern crate rkdb;

use rkdb::{
    api,
    kbindings::*
};

struct KDBStore {

}

impl Store for KDBStore {
    fn get_order(order_id u64) -> () {

    }

    fn get_order_by_client_oid(&self, client_oid: u64) -> () {

    }

    fn add_order(&self, order: Order) -> () {

    }

    fn update_order(&self, order: Order) -> {

    }

    fn get_position(&self) -> {

    }

    fn update_position(&self, position: Position) -> {

    }

    fn get_margin(&self) -> {

    }

    fn udpate_margin(&self) -> {

    }

    fn get_orderbook(&self) -> {

    }

    fn update_orderbook(&self) -> {

    }

    fn get_trade(&self) -> {

    }

    fn add_trade(&self) -> {

    }

    fn get_instrument(&self) -> {

    }

    fn update_instrument(&self) -> {

    }

    fn get_observation_vector(&self) -> {

    }
}
