pub struct Player {
    items: Vec<String>,
    //..
}

pub struct InvulnerablePlayer {
    items: Vec<String>,
    //..
}

fn main() {
    //1 - Should construct a new Player object from name and health, and print debug info

    let mut mario = Player::new(100);
    assert_eq!(format!("{:?}", mario), "Player { items: [], health: 100 }");
    
    //2 - Should reduce health by specified amount

    // mario.take_damage(10);
    // assert_eq!(format!("{:?}", mario), "Player { items: [], health: 90 }");

    //3 - Should work with different values

    // let mut luigi = Player::new(200);
    // assert_eq!(format!("{:?}", luigi), "Player { items: [], health: 200 }");
    // luigi.take_damage(100);
    // assert_eq!(format!("{:?}", luigi), "Player { items: [], health: 100 }");

    //4 - Should place picked up items in inventory

    // assert_eq!(mario.no_of_items_carried(), 0);
    // mario.pick_up_item("Super Star");
    // assert_eq!(mario.no_of_items_carried(), 1);
    // assert_eq!(format!("{:?}", mario), "Player { items: [\"Super Star\"], health: 90 }");

    //5 - When eating a super star Mario should become invulnerable, and invalidate old player object
    // by taking ownership of self in the `eat` method

    // let invulnerable_mario = mario.eat();
    // assert_eq!(format!("{:?}", invulnerable_mario), "InvulnerablePlayer { items: [], health: 90 }");
    // //Note that by consuming `mario` we now cannot access it. Because Mario cannot take damage after
    // //eating a Super Star we do not implement the `take_damage` function in `InvulnerablePlayer`. This way
    // //we can use Rust's type system to enforce an invariant. If anyone tries to apply damage to Mario
    // //in his in invulnerable form the compiler will prevent that bug before it reaches run-time.
    // //This is the power of Rust's type system and ownership!
    // //Uncommenting the following line should cause a compile error:
    // // let test = mario.take_damage(10);

    //6 - given time has run out, invulnerable player becomes regular player, and invalidate

    // //invulnerable player by taking ownership of self in `lose_invulnerability` method
    // let mario = invulnerable_mario.lose_invulnerability();
    // assert_eq!(format!("{:?}", mario), "Player { items: [], health: 90 }");
    // //Uncommenting the following line should cause a compile error:
    // // let test = invulnerable_mario;
}
