mod init;
mod transform;
mod spawn_workers;
pub use self::init::init;
pub use self::transform::transform;
pub use self::spawn_workers::spawn_workers;

#[derive(Debug)]
pub struct TransformedRecord {
    pub id: String,
    /* Seconds since unix epoch */
    pub timestamp: i64,
    pub value: f64,
    pub tag: String,
    /* 1 if value > 0, else 0
    * ∙ using an integer as it makes representation simple.
    * ∙ bool is 1 byte as well.
    * ∙ drawback is we cannot cast u8 -> bool in a hypothetical future requirement.
    * */
    pub positive: u8,
}
