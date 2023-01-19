#[allow(dead_code)]
pub struct Vocabulary {
    words: Vec<&'static str>,
}

#[allow(dead_code)]
impl Vocabulary {
    pub fn new() -> Self {
        return Vocabulary {
            words: vec!["A","Akesi","Ala","Alasa","Ale","Anpa","Ante","Anu","Awen","E","Esun","Ijo","Ike","Ilo","Insa","Jaki","Jan","Jelo","Jo","Kala","Kalama","Kama","Kasi","Ken","Kepeken","Kili","Kin","Kipisi","Kiwen","Ko","Kon","Kule","Kulupu","Kute","La","Lape","Laso","Lawa","Len","Lete","Li","Lili","Linja","Lipu","Loje","Lon","Luka","Lukin","Lupa","Ma","Mama","Mani","Meli","Mi","Mije","Moku","Moli","Monsi","Mu","Mun","Musi","Mute","Namako","Nanpa","Nasa","Nasin","Nena","Ni","Nimi","Noka","O","Oko","Olin","Ona","Open","Pakala","Pali","Palisa","Pan","Pana","Pi","Pilin","Pijema","Pini","Pipi","Poka","Poki","Pona","Pu","Sama","Seli","Selo","Seme","Sewi","Sijelo","Sike","Sin","Sina","Sinpin","Sitelen","Sona","Soweli","Suli","Suno","Supa","Suwi","Tan","Taso","Tawa","Telo","Tenpo","Toki","Tomo","Tu","Unpa","Uta","Utala","Walo","Wan","Waso","Wawa","Weka","Wile"]
        }
    }

    pub fn print_vocabulary(&self) {
        for v in 0..self.words.len() {
            println!("{}",self.words[v]);
        }
    }
}