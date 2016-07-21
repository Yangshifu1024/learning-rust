#![feature(inclusive_range_syntax)]
#![feature(box_syntax, box_patterns)]
mod variables;
use variables::variables;
//
mod mutation;
use mutation::mutations;
//
mod destruction;
use destruction::destruction;
//
mod tuple;
use tuple::tuple;
//
mod array;
use array::array;
//
mod vec;
use vec::vec;
//
mod string;
use string::string;
//
mod structs;
use structs::structs;
// #![feature(inclusive_range_syntax)]
mod for_loop;
use for_loop::for_loop;
//
mod match_pattern;
use match_pattern::match_pattern;
//
mod functions;
use functions::functions;
//
mod boxes;
use boxes::boxes;
fn main() {
    variables();
    mutations();
    destruction();
    tuple();
    array();
    vec();
    string();
    structs();
    for_loop();
    match_pattern();
    functions();
    boxes();
}
