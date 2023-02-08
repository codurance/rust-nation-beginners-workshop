pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

impl Player {
    pub fn revive(&self) -> Option<Player> {
        unimplemented!("Revive this player")
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        unimplemented!("Cast a spell of cost {}", mana_cost)
    }
}

fn main() {
    //=== 1, revive dead player

    let dead_player = Player {
        health: 0,
        mana: Some(0),
        level: 34,
    };
    let revived_player = dead_player
        .revive()
        .expect("reviving a dead player must return Some(player)");
    assert_eq!(revived_player.health, 100);
    assert_eq!(revived_player.mana, Some(100));
    assert_eq!(revived_player.level, dead_player.level);

    //=== 2, revive dead level 9 player without mana

    // let dead_player = Player {
    //     health: 0,
    //     mana: None,
    //     level: 9,
    // };
    // let revived_player = dead_player
    //     .revive()
    //     .expect("reviving a dead player must return Some(player)");
    // assert_eq!(revived_player.health, 100);
    // assert_eq!(revived_player.mana, None);
    // assert_eq!(revived_player.level, dead_player.level);

    //=== 3, revive dead level 10 player with mana

    // let dead_player = Player {
    //     health: 0,
    //     mana: Some(0),
    //     level: 10,
    // };
    // let revived_player = dead_player
    //     .revive()
    //     .expect("reviving a dead player must return Some(player)");
    // assert_eq!(revived_player.health, 100);
    // assert_eq!(revived_player.mana, Some(100));
    // assert_eq!(revived_player.level, dead_player.level);

    //=== 4, reviving alive player should return `None`

    // let alive_player = Player {
    //     health: 1,
    //     mana: None,
    //     level: 8,
    // };
    // assert!(alive_player.revive().is_none());

    //=== 5, casting a spell does mana cost times two damage and reduces mana

    // let health: u32 = 99;
    // let mana: u32 = 100;
    // let level: u32 = 100;
    // let mana_cost: u32 = 3;
    //
    // let mut accomplished_wizard = Player {
    //     health,
    //     mana: Some(mana),
    //     level,
    // };
    //
    // assert_eq!(accomplished_wizard.cast_spell(mana_cost), mana_cost * 2);
    // assert_eq!(accomplished_wizard.health, health);
    // assert_eq!(accomplished_wizard.mana, Some(mana - mana_cost));
    // assert_eq!(accomplished_wizard.level, level);

    //=== 6, cast spell w/ insufficient mana fails

    // let mut no_mana_wizard = Player {
    //     health: 56,
    //     mana: Some(2),
    //     level: 22,
    // };
    //
    // let clone = Player { ..no_mana_wizard };
    //
    // assert_eq!(no_mana_wizard.cast_spell(3), 0);
    // assert_eq!(no_mana_wizard.health, clone.health);
    // assert_eq!(no_mana_wizard.mana, clone.mana);
    // assert_eq!(no_mana_wizard.level, clone.level);

    //=== 7, cast with no mana pool fails and costs health instead of mana

    // let mana_cost: u32 = 10;
    //
    // let mut underleveled_player = Player {
    //     health: 87,
    //     mana: None,
    //     level: 6,
    // };
    //
    // let clone = Player {
    //     ..underleveled_player
    // };
    //
    // assert_eq!(underleveled_player.cast_spell(mana_cost), 0);
    // assert_eq!(underleveled_player.health, clone.health - mana_cost);
    // assert_eq!(underleveled_player.mana, clone.mana);
    // assert_eq!(underleveled_player.level, clone.level);

    //=== 8, cast large spell with no mana pool

    // const MANA_COST: u32 = 30;
    //
    // let mut underleveled_player = Player {
    //     health: 20,
    //     mana: None,
    //     level: 6,
    // };
    //
    // assert_eq!(underleveled_player.cast_spell(MANA_COST), 0);
    // assert_eq!(underleveled_player.health, 0);
    // assert_eq!(underleveled_player.mana, None);
    // assert_eq!(underleveled_player.level, 6);
}
