use crate::NORMAL_TEXT;

#[derive(Copy, Clone)]
pub struct ScreenChar {
    pub character: u32,
    pub position: (i32, i32),
    pub color: i16, // color pair number
}

pub struct MultiLine {
    characters: Vec<ScreenChar>,
} 

impl MultiLine {
    pub fn new(text: String, line_length: i32) -> MultiLine {
        let mut v: Vec<ScreenChar> = Vec::with_capacity(text.chars().count());

        let mut sc = ScreenChar {character: 0, position: (0, 0), color: NORMAL_TEXT};
        for word in text.split(" ") {
            if sc.position.1 + word.len() as i32 >= line_length {
                // next line
                sc.position.1 = 0;
                sc.position.0 += 1;
            }

            // do as usual
            for ch in word.chars().chain(Some(' ').into_iter()) {
                sc.character = ch.into();
                v.push(sc);
                sc.position.1 += 1;
            }
        }

        MultiLine { characters: v }
    }

    pub fn len(&self) -> usize {
        self.characters.len()
    }

    pub fn get(&self, index: usize) -> &ScreenChar {
        &self.characters[index]
    }

    pub fn get_mut(&mut self, index: usize) -> &mut ScreenChar {
        &mut self.characters[index]
    }
}
