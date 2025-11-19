use ::rustle::*;
fn main() {
    println!("Hello, world!");
    data_generator::init_extract();
    extract::init_extract();
    transform::init_transform();
    load::init_load(); 
}
