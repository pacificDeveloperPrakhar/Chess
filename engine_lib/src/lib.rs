
#![deny(clippy::all)]
mod evaluator;
mod validator;
use evaluator::moves_evaluator;
use validator::epd_to_num;
#[macro_use]
extern crate napi_derive;

#[napi]
pub engine_process(epd:String,move_str:String,color_str:String){
  if color_str=='a'{
    move_str
  }
}