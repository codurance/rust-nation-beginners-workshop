enum Item {
    HealthPotion(u8),
    PoisonPotion(u8)
}

struct Inventory {
    items: Vec<Item>,
}

impl Inventory {
    fn new() -> Inventory {
        unimplemented!();
    }

    fn push(&mut self, item: Item) {
        unimplemented!();
    }

    fn get(&mut self, index: usize) -> Option<Item> {
        unimplemented!();
    }

    fn len(&self) -> usize {
       unimplemented!();
    }
}

fn main() {
    let mut player_inventory = Inventory::new();
    player_inventory.push(Item::HealthPotion(10));
    player_inventory.push(Item::PoisonPotion(5));

    let health_potion = player_inventory.get(0).unwrap();
    //Assert that health_potion is a health potion for 10 HP
    assert!(matches!(health_potion, Item::HealthPotion(10)));
    assert_eq!(1, player_inventory.len());

    let poison_potion = player_inventory.get(0).unwrap();
    //Assert that poison_potion is a poison potion for 5 HP
    assert!(matches!(poison_potion, Item::PoisonPotion(5)));
    assert_eq!(0, player_inventory.len());

    let nothing = player_inventory.get(0);
    assert!(matches!(nothing, None));
}
