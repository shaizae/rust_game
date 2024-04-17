pub struct Caricature {
    pub life: i32,
    pub attack: u16,
    pub defense: u16,

}

impl Caricature {
    pub fn hit(&mut self, mut damage: u16) {
        damage = damage - self.defense;
        if damage > 0 {
            self.life = self.life - damage as i32;
        }
        if self.life.is_negative() || self.life==0{self.die();}

    }
    fn die(&mut self){
        // TODO: set Caricature to die
    }

}