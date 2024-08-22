/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use surreal_models::{Rocket, Weapon};
use surreal_orm::PartialUpdater;
use serde_json;

// type Lala<'a, T> = <Weapon<'a, T> as PartialUpdater>::StructPartial;

#[test]
fn can_do_partial_update() {
    let rocket = Rocket::partial_builder()
        .strength(907)
        .name("Ye".into())
        .build();

    let weapon = Weapon::partial_builder()
        .name("Oyelowo".into())
        .strength(45.0)
        .rocket(rocket)
        .build();

    println!("{:?}", serde_json::to_string(&weapon).unwrap());
    // assert_eq!(weapon.name, "");

    // assert_eq!(serde_json::to_string(&weapon).unwrap(), "");
    // assert_eq!(serde_json::to_string(&weapon).unwrap(), r#"{"name":"Oyelowo","strength":45.0,"rocket":{"name":"Ye","strength":907}}");
    // assert_eq!(weapon, "");
}
