// #![allow(dead_code)]
// #![allow(unused_imports)]
// #![feature(extern_prelude)]

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
// use serde_json::*;
mod gas_libs;
use gas_libs::gas_validation::gas_validation as gv;
use gas_libs::static_variables_form::static_variables_form as STATIC;
// use std::rc::Rc;

extern crate cursive;
// use cursive::align::{HAlign, VAlign};
// use cursive::traits::*;
// use cursive::view::{Offset, Position};
// use cursive::views::*;
use cursive::Cursive;

extern crate chrono;
// use chrono::prelude::*;

// use std::env;
// use std::fs::File;
// use std::io::Read;

// Start gui + logic
fn cursive_linear() {
    let mut siv = Cursive::default();

    gv::gas_entry_dialog(&mut siv);
    gv::populate_form_with_date_and_c_price(&mut siv);

    siv.run();
}

fn main() {
    cursive_linear();
    let gas_entries = gv::get_gas_entries(STATIC::GAS_DATA_JSON_FILE);
    let len10 = gas_entries.len() - 10;
    let last_10_entries: Vec<gv::GasEntry> = gas_entries[len10..].iter().cloned().collect();

    for g in last_10_entries.iter() {
        println!(
            "{:.16} {} {} {} {:.2}",
            g.date, g.cMeters, g.consumed, g.cPrice, g.total
        );
    }
}
