#![feature(simd)]
#![allow(non_camel_case_types)]

//! Definitions of many SIMD types of fixed lengths.

include!(concat!(env!("OUT_DIR"), "/types.rs"));
