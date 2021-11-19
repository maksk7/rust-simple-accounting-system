pub struct Tax {
    name: String,
    percent: f32,
}

impl Tax {
    pub async fn new(name: String, percent: f32) -> Self {
        Tax { name, percent }
    }

    pub async fn change_name(&mut self, name: String) {
        self.name = name;
    }

    pub async fn change_percent(&mut self, percent: f32) {
        self.percent = percent;
    }
}
