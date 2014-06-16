#[crate_id = "iron-router"]
#![license = "MIT"]
//#![deny(missing_doc)]
#![deny(unused_result, unused_result, unnecessary_qualification,
        non_camel_case_types, unused_variable, unnecessary_typecast)]
#![feature(phase, macro_export)]

extern crate http;
extern crate iron;
extern crate regex;
#[phase(plugin)] extern crate regex_macros;

pub use router::Router;

mod router;