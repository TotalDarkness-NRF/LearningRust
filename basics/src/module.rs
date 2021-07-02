/*
Modules allow us to organize projects better
This file acts as a module
By defualt a module will be private (Its functions can not be accssed)
We can use pub to make a particular function public
In order to include this module into another file we use mod <name of file>;
We can also use "use crate::<file name>::<module name>;" for specific modules
*/
fn privateFunction() {
    println!("This is a private function");
}
pub fn publicFunction() {
    privateFunction();
    println!("This is a public function");
}

// We can have other functions and modules inside
pub mod anotherModule {
    pub fn moduleFunction() {
        println!("This is a module function");
    }
}