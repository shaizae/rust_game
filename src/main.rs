use caricatures::caricature::Caricature;

mod caricatures;

fn main() {
    let mut hero = Caricature { life: 10, attack: 5, defense: 2 };
    let mut monster = Caricature { life: 15, attack: 5, defense: 2 };
    for _ in 1..10 { monster.get_damage(hero.attack) }
}
