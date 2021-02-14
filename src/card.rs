
#[derive(Clone, Copy, Debug)]
pub enum Face {
    Ace,
    Spade,
    Heart,
    Diamond,
    ERROR
}
impl Face {
    pub fn to_str(face: &Face) -> String {
        match face {
            Face::Ace => String::from("Ace"),
            Face::Spade => String::from("Spade"),
            Face::Heart => String::from("Heart"),
            Face::Diamond => String::from("Diamond"),
            Face::ERROR => String::from("")
        }
    }
    pub fn str_to_face(face: String) -> Face {
        match face.as_str() {
            "Ace" => Face::Ace,
            "Spade" => Face::Spade,
            "Heart" => Face::Heart,
            "Diamond" => Face::Diamond,
            _ => Face::ERROR
        }
    }
}

#[allow(dead_code, unused_variables)]
#[derive(Clone, Copy, Debug)]
pub struct Card {
    face: Face,
    num: u8,
}
#[allow(dead_code, unused_variables)]
impl Card {
    pub fn init(face: String, num: u8) -> Self {
        Card { face: Face::str_to_face(face), num }
    }
    pub fn get_face(&self) -> String {
        Face::to_str(&self.face)
    }
    pub fn get_num(&self) -> u8 {
        self.num
    }
    pub fn comp(self, other: &Card) -> i8 {
        return if self.num < other.num {
            -1
        } else if self.num == other.num {
            0
        } else {
            1
        }
    }
    pub fn print(&self) {
        println!(" struct Card face: {} num: {}\n", Face::to_str(&self.face), &self.num);
    }
}