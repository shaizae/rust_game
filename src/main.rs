mod caricature;


fn main() {
    let mut  hero = caricature::Caricature { life: 10, attack: 5, defense: 2 };
    let mut monster=caricature::Caricature{ life: 10, attack: 5, defense: 2 };
    for _ in 1..10{    monster.get_damage(hero.attack)}


}
