pub trait Creature {
    fn weapons(&self) -> u8;
    fn sound(&self) -> &str {
        return "";
    }
    fn name(&self) -> &str {
        return "";
    }
}

struct Crab {
    claws: u8,
    poison: bool,
}

impl Creature for Crab {
    fn weapons(&self) -> u8 {
        self.claws + if self.poison { 1 } else { 0 }
    }
    fn sound(&self) -> &str {
        return "None";
    }
    fn name(&self) -> &str {
        return "Crab";
    }
}

struct Dog {
    claws: u8,
    fang: bool,
}

impl Creature for Dog {
    fn weapons(&self) -> u8 {
        self.claws + if self.fang { 1 } else { 0 }
    }
    fn sound(&self) -> &str {
        return "Bow";
    }
    fn name(&self) -> &str {
        return "Dog";
    }
}

fn main() {
    let crab = Crab {
        claws: 2,
        poison: true,
    };
    let dog = Dog {
        claws: 4,
        fang: true,
    };

    println!(
        "name: {}, weapons: {}, sound: {}",
        crab.name(),
        crab.weapons(),
        crab.sound()
    );
    println!(
        "name: {}, weapons: {}, sound: {}",
        dog.name(),
        dog.weapons(),
        dog.sound()
    );
}
