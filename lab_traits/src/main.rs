#[derive(Debug)]
enum Element {
  Light,
  Dark,
  Space,
  Time,
  Void
}

#[derive(Debug)]
enum Race {
  Divine,
  Devil,
  Sapien,
  Machine,
  Particle
}

trait Card {
  fn detail(&self);
}

struct Monster {
  name: String,
  element: Element,
  race: Race,
  hp: u16,
  power: u16,
  effect: String
}

struct Support {
  name: String,
  effect: String
}

impl Card for Monster {
 fn detail(&self) {
    println!("Name: {}\nElement: {:?}\nRace: {:?}\nHP: {}\nPower: {}\nEffect: {}", self.name, self.element, self.race, self.hp, self.power, self.effect);
 }
}

impl Card for Support {
  fn detail(&self) {
    println!("Name: {}\nEffect: {}", self.name, self.effect);
  }
}

fn main() {
  let card01 = Monster{
    name: String::from("Light Dragon"),
    element: Element::Light,
    race: Race::Divine,
    hp: 1000,
    power: 100,
    effect: String::from("-")
  };

  let card02 = Support{
    name: String::from("Dragon Fang"),
    effect: String::from("Reduce all non-Divine monster's hp by 100")
  };

  card01.detail();

  card02.detail();

}

