/// Basic elementary cell in data frame
#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub struct Element {
    pub value: String
}

pub trait ElementImpl {
    fn replace(&mut self, new: String);
}

impl ElementImpl for Element {
    fn replace(&mut self, new: String) {
        self.value = new;
    }
}
