extern crate ndarray;

use std::time::SystemTime;
use std::fmt::Debug;
use ndarray::Array1;
use std::cmp;

trait Engine {
    fn run(&self);
    fn stop(&self);
}
