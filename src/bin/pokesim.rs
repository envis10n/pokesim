use pokesim::game::monster::Monster;

fn main() {
    let bulbasaur = Monster::from_dex(1);
    println!("{:?}", bulbasaur);
}