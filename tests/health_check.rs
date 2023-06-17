// ! test.health_check.rs
use newsletter::main;

pub fn dummy_test() {
    // main runs
    let result: Result<(), std::io::Error> = main::main();
    if result.is_err() {
        println!("BADF");
    } else {
        println!("GOODT");
    }
}
