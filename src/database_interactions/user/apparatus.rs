pub struct Apparatus {
    id: String,
    name: String,
    description: String,

    repetitions: u8,
    sets: u8,

    notes: String,
}

impl Apparatus {
    pub fn new(id: String, name: String, description: String) -> Apparatus {
        Apparatus {
            id,
            name,
            description,
            repetitions: 0,
            sets: 0,
            notes: String::new(),
        }
    }
}
