#[macro_use]
extern crate serde_derive;

extern crate chrono;
pub extern crate hyper;
extern crate serde;
extern crate serde_json;
extern crate futures;
extern crate url;
extern crate bigdecimal;

use std::fmt;
use chrono::{Date, NaiveDateTime, DateTime, FixedOffset, Utc, SecondsFormat};

pub mod apis;
pub mod models;
pub mod date_serializer;
pub mod date_serializer_opt;
pub mod datetime_serializer;
pub mod serialize_quoted_numbers;
pub mod serialize_quoted_numbers_opt;

//mod tests;  //Put testing data and token to tests before uncomment

pub(crate) trait OutlinePrint<'a>: fmt::Display {
    fn outline_print(&'a self) -> String {
        format!("{}", self)
    }
}

impl<'a> OutlinePrint<'a> for &'a str {
    fn outline_print(&'a self) -> String {
        format!("{}", self)
    }
}

impl<'a> OutlinePrint<'a> for i64 {
    fn outline_print(&'a self) -> String {
        format!("{:?}", self)
    }
}

impl<'a> OutlinePrint<'a> for f32 {
    fn outline_print(&'a self) -> String {
        format!("{:?}", self)
    }
}

impl<'a> OutlinePrint<'a> for bool {
    fn outline_print(&'a self) -> String {
        format!("{:?}", self)
    }
}
impl<'a> OutlinePrint<'a> for chrono::DateTime<chrono::Utc> {
    fn outline_print(&'a self) -> String {
        format!("{}", self.to_rfc3339_opts(SecondsFormat::Secs, true))
    }
}
impl<'a> OutlinePrint<'a> for chrono::Date<chrono::Utc> {
    fn outline_print(&'a self) -> String {
        self.format("%Y-%m-%d").to_string()
    }
}

