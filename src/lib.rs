// Copyright (c) 2019 E.S.R.Labs. All rights reserved.
//
// NOTICE:  All information contained herein is, and remains
// the property of E.S.R.Labs and its suppliers, if any.
// The intellectual and technical concepts contained herein are
// proprietary to E.S.R.Labs and its suppliers and may be covered
// by German and Foreign Patents, patents in process, and are protected
// by trade secret or copyright law.
// Dissemination of this information or reproduction of this material
// is strictly forbidden unless prior written permission is obtained
// from E.S.R.Labs.

//! # Autosar DLT
//!
//! `dlt-core` contains data structures for DLT messages and related types,
//! a parser to parse binary dlt-messages and a way to create dlt messages

//#![allow(dead_code)]
#[macro_use]
extern crate log;

pub mod dlt;
pub mod dlt_fmt;
pub mod dlt_parse;
pub mod fibex;
pub mod filtering;
pub mod service_id;
pub mod statistics;

#[cfg(test)]
pub mod proptest_strategies;
#[cfg(test)]
mod tests;
