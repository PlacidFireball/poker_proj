
#[allow(dead_code, unused_variables)]
#[derive(Clone, Debug)]
pub struct Card {
    face: String,
    num: u8,
}
#[allow(dead_code, unused_variables)]
impl Card {
    pub fn init(face: String, num: u8) -> Card {
        Card { face, num }
    }
    pub fn get_face(&self) -> &String {
        &self.face
    }
    pub fn get_num(&self) -> u8 {
        self.num
    }
    pub fn print(&self) {
        println!("|-=Card=-\n|\tface: {}\n|\tnum: {}\n", self.face, self.num);
    }
}