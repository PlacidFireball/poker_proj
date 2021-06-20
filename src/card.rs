/*
Face enum - used to keep track of the face of a card
--Types--
Ace, Spade, Diamond, Heart, ERROR (for error handling)
--Traits--
PartialEq, Clone, Copy, Debug
--Methods--
to_str(&Face)           - converts a (borrowed) face to a string
str_to_face(String)     - converts a string to a face
 */
#[derive(PartialEq, Clone, Copy, Debug)]
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


/*
The Card struct, the basis for all other structs
in poker_proj.
--Traits--
Clone, Copy, Debug, Default, PartialEq
--Methods--
init(String, u8)    - initializes a card from a string and and 8bit unsigned integer
comp(Card)          - compares one card to another (based on number) returns -1 if less than,
                        0 if equal and 1 if greater
print()             - prints off debug information to the console
*/
#[allow(dead_code, unused_variables)]
#[derive(Clone, Copy, Debug)]
pub struct Card {
    pub face: Face,
    pub num: u8,
    visible: bool
}
impl Default for Card {
    fn default() -> Card {
        Card { face: Face::ERROR, num: 0, visible: false}
    }
}
impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        self.num == other.num && self.face == other.face
    }
}
#[allow(dead_code, unused_variables)]
impl Card {
    /*  initializes a card from a string and an 8bit unsigned integer*/
    pub fn init(face: String, num: u8) -> Self {
        Card { face: Face::str_to_face(face), num, visible: false }
    }

    /*  compares one card to another*/
    pub fn comp(&self, other: &Card) -> i8 {
        return if self.num < other.num {
            -1
        } else if self.num == other.num {
            0
        } else {
            1
        }
    }
    /*  prints debug information to the console*/
    pub fn print(&self) {
        println!("struct Card face: {} num: {} visible: {}", Face::to_str(&self.face), &self.num, &self.visible);
    }
}