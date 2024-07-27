pub struct Queue {
    pub _id: u8,
    pub platforms: Vec<Platform>,
}

pub struct Platform {
    pub _id: u8,
    pub spawn1: String,
    pub spawn2: String,
}
