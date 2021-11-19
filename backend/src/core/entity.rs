pub trait Entity {
    fn id(&self) -> u32;
    fn new(self) -> Result<Ok(Self), Err>;
    fn validate(&self) -> Result<Ok, Err>;
}
