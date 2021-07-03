#![allow(non_snake_case)] // stop warning for not using snake case

mod module;
mod variables;
mod controlFlow;
mod compoundTypes;
mod loops;
mod ownership;
mod structures;
use crate::module::anotherModule;

fn main() {
    variables::variables();
    controlFlow::ifelse();
    compoundTypes::compoundTypes();
    loops::loops();
    ownership::ownership();
    structures::structs();
    module::publicFunction();
    module::anotherModule::moduleFunction();
    anotherModule::moduleFunction();
}

/*
Functions are declared with fn and wrapped in {}
They can either be a statment or expression. A statment returns nothing while expression returns a value
Paramenters are declared in the brackets using variable: type.
Return values are specified using -> type after the brackets and before the {}
A final return does not require a semi colon. Use this if its the final return
 */