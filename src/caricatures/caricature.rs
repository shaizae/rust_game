

pub struct Caricature {
    pub life: i32,
    pub attack: u16,
    pub defense: u16,

}

impl Caricature {
    pub fn get_damage(&mut self, mut damage: u16) {
        damage = damage - self.defense;
        if damage > 0 {
            self.life = self.life - damage as i32;
        }
        if self.life.is_negative() || self.life==0{self.die();}
        else { println!("{}",self.life) }

    }
    fn die(&mut self){
        println!("i am already dead")
    }

}