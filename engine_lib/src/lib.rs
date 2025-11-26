
#![deny(clippy::all)]
mod evaluator;
mod validator;
use evaluator::moves_evaluator;
use validator::epd_to_num;
#[macro_use]
extern crate napi_derive;

