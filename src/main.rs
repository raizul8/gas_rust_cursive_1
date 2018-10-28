#![allow(dead_code)]
#![allow(unused_imports)]

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
use serde_json::*;
mod gas_libs;
use gas_libs::gas_validation::gas_validation as fv;
use gas_libs::static_variables_form::static_variables_form as STATIC;
use std::rc::Rc;

extern crate cursive;
use cursive::align::{HAlign, VAlign};
use cursive::traits::*;
use cursive::view::{Offset, Position};
use cursive::views::*;
use cursive::Cursive;

extern crate chrono;
use chrono::prelude::*;

use std::fs::File;
use std::io::Read;

fn cursive_linear() {
    let mut siv = Cursive::default();

    fv::gas_entry_dialog(&mut siv);
    fv::populate_form_with_date_and_c_price(&mut siv);

    siv.run();
}

fn main() {
    // println!("gas: {}", gas_lib1::gas());
    cursive_linear();
}
