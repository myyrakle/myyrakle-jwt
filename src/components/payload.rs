#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct Payload<T> {
    exp: u64,
    data: T,
}

impl<T> Payload<T> {
    pub fn new(exp: u64, data: T) -> Payload<T> {
        Payload { exp, data }
    }

    pub fn get_data(self) -> T {
        self.data
    }
}
