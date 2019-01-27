extern crate chrono;
use chrono::prelude::*;

// use std::error::Error;
use std::fmt;
use std::fs;
use std::fs::File;
use std::io::Read;
// use std::env;
extern crate serde;
extern crate serde_json;

extern crate cursive;
use cursive::align::{HAlign};
use cursive::traits::*;
// use cursive::view::{Offset, Position};
use cursive::views::*;
use cursive::Cursive;

use gas_libs::static_variables_form::static_variables_form as STATIC;

fn validate_form_on_submit(s: &mut Cursive) {
    match validate_form(s) {
        Ok(gas_input) => {
            let msg = format!("Form validated and saved on disk. \n{}", gas_input);
            s.add_layer(Dialog::info(msg));
        }
        Err(err) => s.add_layer(Dialog::info(err)),
    }
}

fn delete_last_entry(s: &mut Cursive) {
    let gas_entries = get_gas_entries(STATIC::GAS_DATA_JSON_FILE);
    let last_entry = gas_entries[gas_entries.len() - 1].clone();
    let del_msg = format!("Delete last entry from file ? \n{:#?}", last_entry);

    s.add_layer(
        Dialog::text(del_msg)
            .button("Yes", move |s| {
                let mut gas_entries = gas_entries.clone();
                gas_entries.pop();
                let gas_entries_json = serde_json::to_string_pretty(&gas_entries).unwrap();

                fs::write(STATIC::GAS_DATA_JSON_FILE, &gas_entries_json).expect(&format!(
                    "\n- File: {} \n- line: {} \n- err: {}\n",
                    file!(),
                    line!(),
                    "Could not write to file"
                ));

                s.pop_layer();
            })
            .button("No", |s| {
                s.pop_layer();
            }),
    );
}

fn validate_form(s: &mut Cursive) -> Result<GasEntry, String> {
    let mut gas_entries = get_gas_entries(STATIC::GAS_DATA_JSON_FILE);
    let len_gas_entries = gas_entries.len();
    let last_gas: GasEntry = (gas_entries[len_gas_entries - 1]).clone();
    let last_date = DateTime::parse_from_rfc3339(&last_gas.date)
        .expect("Could not parse last date from gas_entries");

    let gas_input: i32;
    match get_form_gas_input(s) {
        Ok(ok) => gas_input = ok,
        Err(err) => return Err(err),
    }

    match validate_gas_input_on_submit(gas_input, last_gas.cMeters) {
        Ok(_) => {}
        Err(err) => return Err(err),
    }

    let form_date: DateTime<chrono::FixedOffset>;
    match get_form_date(s) {
        Ok(date) => form_date = date,
        Err(err) => return Err(err),
    }

    match validate_date_on_form_submit(form_date, last_date) {
        Ok(_) => {}
        Err(err) => return Err(err),
    }

    let c_price: f64;
    match get_form_cubic_meter_price(s) {
        Ok(ok) => c_price = ok,
        Err(err) => return Err(err),
    }

    match validate_cubic_meter_price_on_submit(c_price) {
        Ok(_) => {}
        Err(err) => return Err(err),
    }

    let consumed = gas_input - last_gas.cMeters;

    let valid_form_gas_entry = GasEntry {
        date: form_date.to_rfc3339_opts(SecondsFormat::Millis, true),
        cMeters: gas_input,
        consumed,
        cPrice: c_price,
        total: consumed as f64 * c_price,
    };

    let res_valid_form_gas_entry = valid_form_gas_entry.clone();

    gas_entries.push(valid_form_gas_entry);

    let gas_entries_json = serde_json::to_string_pretty(&gas_entries).unwrap();

    fs::write(STATIC::GAS_DATA_JSON_FILE, &gas_entries_json).expect(&format!(
        "\n- File: {} \n- line: {} \n- err: {}\n",
        file!(),
        line!(),
        "Could not write to file"
    ));

    Ok(res_valid_form_gas_entry)
}

fn get_form_gas_input(s: &mut Cursive) -> Result<i32, String> {
    let gas_s = s
        .call_on_id(STATIC::GAS_CUBIC_METERS_ID, |view: &mut EditView| {
            view.get_content()
        })
        .unwrap();

    let gas_amount = gas_s.parse::<i32>();

    match gas_amount {
        Ok(nr) => Ok(nr),
        Err(_) => Err(format!("Could not parse string to whole number",)),
    }
}

fn get_form_cubic_meter_price(s: &mut Cursive) -> Result<f64, String> {
    let gas_price_s = s
        .call_on_id(STATIC::GAS_CUBIC_PRICE_ID, |view: &mut EditView| {
            view.get_content()
        })
        .unwrap();

    let gas_price = gas_price_s.parse::<f64>();

    match gas_price {
        Ok(nr) => Ok(nr),
        Err(_) => Err(format!(
            "Gas price input: {} could not be parsed to float number.",
            gas_price_s
        )),
    }
}

fn get_form_date(s: &mut Cursive) -> Result<DateTime<chrono::FixedOffset>, String> {
    let date_s = s
        .call_on_id(STATIC::GAS_DATE_ID, |view: &mut EditView| {
            view.get_content()
        })
        .unwrap()
        .to_string();

    let rfc3339 = DateTime::parse_from_rfc3339(&date_s);

    match rfc3339 {
        Ok(ok) => Ok(ok),
        Err(_) => {
            let custom_err = format!(
                "Input: {} is invalid format. \nValid ex: 2014-03-15T11:49:03.541Z",
                date_s
            );
            return Err(String::from(custom_err));
        }
    }
}

fn validate_gas_input_on_submit(input_gas: i32, last_gas: i32) -> Result<(), String> {
    if input_gas <= last_gas {
        return Err(format!(
            "Input {} must be greater than {}.",
            input_gas, last_gas
        ));
    }

    Ok(())
}

fn validate_cubic_meter_price_on_submit(form_cm_price: f64) -> Result<(), String> {
    if form_cm_price <= 0.0 {
        return Err(format!(
            "Cubic price cannot be negative or zero {}",
            form_cm_price
        ));
    }

    Ok(())
}

fn validate_date_on_form_submit(
    form_date: DateTime<chrono::FixedOffset>,
    last_date: DateTime<chrono::FixedOffset>,
) -> Result<(), String> {
    // input year cannot be lower than previous year
    if form_date.year() < last_date.year() {
        return Err(format!(
            "Inserted year {} must be greater than {}",
            form_date.year(),
            last_date.year()
        ));
    }
    // if years are equal current month has to be bigger than previous month by one
    if form_date.year() == last_date.year() && form_date.month() != last_date.month() + 1 {
        return Err(format!(
            "Input month {} has to be greater by one than previous month {}",
            form_date.month(),
            last_date.month()
        ));
    }
    // if input year is bigger than previous year
    // it can only be greater by one year and
    // input month must be 1
    if form_date.year() > last_date.year() {
        if form_date.year() - 1 != last_date.year() {
            return Err(format!(
                "Input year({}) can only be greater by one year from previous year({}) !",
                form_date.year(),
                last_date.year()
            ));
        }

        // to insert new year previous month has to be 12 and current month has to be 1
        if last_date.month() != 12 {
            return Err(format!(
                "The month of last entry must be 12 not {}.",
                last_date.month(),
            ));
        }
        // to insert new year month has to be 01
        if form_date.month() != 1 {
            return Err(format!("Input month is {}, must be 01.", form_date.month()));
        }
    }

    // input date must be between 15 and 20
    if form_date.day() < 15 || form_date.day() > 20 {
        return Err(format!(
            "Input day must be between 15 and 20. \nInserted {} .",
            form_date.day()
        ));
    }

    return Ok(());
}

pub fn populate_form_with_date_and_c_price(s: &mut Cursive) {
    let gas_entries = get_gas_entries(STATIC::GAS_DATA_JSON_FILE);
    let last_entry = &gas_entries[gas_entries.len() - 1];

    s.call_on_id(STATIC::GAS_DATE_ID, |view: &mut EditView| {
        view.set_content(
            Utc::now()
                .to_rfc3339_opts(SecondsFormat::Millis, true)
                .to_string(),
        );
    });

    s.call_on_id(STATIC::GAS_CUBIC_PRICE_ID, |view: &mut EditView| {
        view.set_content(last_entry.cPrice.to_string());
    });
}

// check functions are just checking for correct format ex: gas input date is an actual date,
// gas input is a whole number.

fn check_gas_field(s: &mut Cursive, _gas_amount: &str) {
    match get_form_gas_input(s) {
        Ok(_) => {}
        Err(err) => s.add_layer(Dialog::info(err)),
    }
}

fn check_cubic_meter_field(s: &mut Cursive, _cubic_meter_str: &str) {
    match get_form_cubic_meter_price(s) {
        Ok(_) => {}
        Err(err) => s.add_layer(Dialog::info(err)),
    }
}

fn check_date_field(s: &mut Cursive, _date: &str) {
    match get_form_date(s) {
        Ok(_) => {}
        Err(err) => s.add_layer(Dialog::info(err)),
    }
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GasEntry {
    pub date: String,
    pub cMeters: i32,
    pub consumed: i32,
    pub cPrice: f64,
    pub total: f64,
}

impl fmt::Display for GasEntry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "GasEntry {{
    cMeters: {} 
    Date: {} 
    consumed: {} 
    cPrice: {} 
    total: {}
}}",
            self.cMeters, self.date, self.consumed, self.cPrice, self.total
        )
    }
}

pub fn get_gas_entries(file: &str) -> Vec<GasEntry> {
    let mut json_file: File = File::open(file).expect(&format!(
        "\n- File: {} \n- line: {} \n- err: {}\n",
        file!(),
        line!(),
        ""
    ));
    let mut contents = String::new();

    json_file
        .read_to_string(&mut contents)
        .expect("Could not read gas_data.json");

    let err_message = &format!(
        "\n- File: {} \n- line: {} \n- err: {}\n",
        file!(),
        line!(),
        "Json file could not be parsed"
    );

    let gas_entries: Vec<GasEntry> = serde_json::from_str(&contents).expect(err_message);

    gas_entries
}

pub fn gas_entry_dialog(siv: &mut Cursive) {
    siv.add_layer(
        Dialog::around(
            LinearLayout::vertical()
                .child(TextView::new("Gas Form").h_align(HAlign::Center))
                .child(DummyView.fixed_height(1))
                // .child(TextView::new(text).scrollable())
                .child(
                    Dialog::new()
                        .title("Total cubic meters:")
                        .padding((1, 1, 1, 0))
                        .content(
                            EditView::new()
                                .on_submit(check_gas_field)
                                .with_id(STATIC::GAS_CUBIC_METERS_ID)
                                .fixed_width(35),
                        ),
                )
                .child(
                    Dialog::new().title("Date:").padding((1, 1, 1, 0)).content(
                        EditView::new()
                            .on_submit(check_date_field)
                            .with_id(STATIC::GAS_DATE_ID),
                    ),
                )
                .child(
                    Dialog::new()
                        .title("Enter cubic meter price:")
                        .padding((1, 1, 1, 0))
                        .content(
                            EditView::new()
                                .on_submit(check_cubic_meter_field)
                                .with_id(STATIC::GAS_CUBIC_PRICE_ID),
                        ),
                ),
        )
        .button("Insert", |s| validate_form_on_submit(s))
        .button("Delete Last Entry", |s| delete_last_entry(s))
        .button("Quit", |s| s.quit())
        .h_align(HAlign::Center),
    );
    populate_form_with_date_and_c_price(siv);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t_validate_form() {
        // let mut gas_entries = get_gas_entries(STATIC::GAS_DATA_JSON_FILE);
        // let len_gas_entries = gas_entries.len();
        // let last_gas: &mut GasEntry = &mut gas_entries[len_gas_entries - 1];
        let mut gas_entries: Vec<GasEntry> = vec![
            GasEntry {
                date: String::from("2018-08-16T10:14:00.000Z"),
                cMeters: 4072,
                consumed: 95,
                cPrice: 1.65,
                total: 156.75,
            },
            GasEntry {
                date: String::from("2018-09-15T10:13:00.000Z"),
                cMeters: 4171,
                consumed: 99,
                cPrice: 1.65,
                total: 163.35,
            },
            GasEntry {
                date: String::from("2018-10-16T10:14:00.000Z"),
                cMeters: 4248,
                consumed: 177,
                cPrice: 1.65,
                total: 292.05,
            },
        ];
        let len_gas_entries = gas_entries.len();
        let last_gas: &mut GasEntry = &mut gas_entries[len_gas_entries - 1];

        let res = validate_gas_input_on_submit(4251, last_gas.cMeters);
        assert_eq!(res, Ok(()));

        let v = 3000;
        let res = validate_gas_input_on_submit(v, last_gas.cMeters);
        assert_eq!(
            res,
            Err(format!(
                "Input {} must be greater than {}.",
                v, last_gas.cMeters
            ))
        );
    }

    #[test]
    fn t_validate_gas_input_on_submit() {
        let res = validate_gas_input_on_submit(4251, 4248);
        assert_eq!(res, Ok(()));
    }

    #[test]
    fn t_validate_date_on_form_submit() {
        // Form date correct
        let last_date = DateTime::parse_from_rfc3339("2018-10-16T10:14:00.000Z").unwrap();
        let form_date = DateTime::parse_from_rfc3339("2018-11-16T10:14:00.000Z").unwrap();
        let res = validate_date_on_form_submit(form_date, last_date);

        // input year cannot be lower than previous year
        assert_eq!(res, Ok(()));
        let last_date = DateTime::parse_from_rfc3339("2018-10-16T10:14:00.000Z").unwrap();
        let form_date = DateTime::parse_from_rfc3339("2015-11-16T10:14:00.000Z").unwrap();
        let res = validate_date_on_form_submit(form_date, last_date);

        let exp = Err(format!(
            "Inserted year {} must be greater than {}",
            form_date.year(),
            last_date.year()
        ));

        assert_eq!(res, exp);

        //if years are equal current month has to be bigger than previous month by one
        let last_date = DateTime::parse_from_rfc3339("2018-10-16T10:14:00.000Z").unwrap();
        let form_date = DateTime::parse_from_rfc3339("2018-12-16T10:14:00.000Z").unwrap();
        let res = validate_date_on_form_submit(form_date, last_date);

        let this_err = Err(format!(
            "Input month {} has to be greater by one than previous month {}",
            form_date.month(),
            last_date.month()
        ));

        assert_eq!(res, this_err);

        // if input year is bigger than previous year
        // it can only be greater by one year
        let last_date = DateTime::parse_from_rfc3339("2018-12-16T10:14:00.000Z").unwrap();
        let form_date = DateTime::parse_from_rfc3339("2022-01-16T10:14:00.000Z").unwrap();
        let res = validate_date_on_form_submit(form_date, last_date);

        let exp = Err(format!(
            "Input year({}) can only be greater by one year from previous year({}) !",
            form_date.year(),
            last_date.year()
        ));

        assert_eq!(res, exp);

        // to insert new year previous month has to be 12 and current month has to be 1
        let last_date = DateTime::parse_from_rfc3339("2018-11-16T10:14:00.000Z").unwrap();
        let form_date = DateTime::parse_from_rfc3339("2019-01-16T10:14:00.000Z").unwrap();
        let res = validate_date_on_form_submit(form_date, last_date);

        let exp = Err(format!(
            "The month of last entry must be 12 not {}.",
            last_date.month(),
        ));

        assert_eq!(res, exp);

        // to insert new year month has to be 01
        let last_date = DateTime::parse_from_rfc3339("2018-12-16T10:14:00.000Z").unwrap();
        let form_date = DateTime::parse_from_rfc3339("2019-02-16T10:14:00.000Z").unwrap();
        let res = validate_date_on_form_submit(form_date, last_date);

        let exp = Err(format!("Input month is {}, must be 01.", form_date.month()));

        assert_eq!(res, exp);

        // input date must be between 15 and 20
        let last_date = DateTime::parse_from_rfc3339("2018-12-16T10:14:00.000Z").unwrap();
        let form_date = DateTime::parse_from_rfc3339("2019-02-16T10:14:00.000Z").unwrap();
        let res = validate_date_on_form_submit(form_date, last_date);

        let exp = Err(format!("Input month is {}, must be 01.", form_date.month()));

        assert_eq!(res, exp);
    }
}
