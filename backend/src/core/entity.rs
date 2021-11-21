pub trait Entity {
    fn get_id(&self) -> &u32;
    fn set_id(&mut self, id: u32);
}
