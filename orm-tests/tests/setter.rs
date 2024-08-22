/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use pretty_assertions::assert_eq;
use surreal_models::{Organization, Time};
use surreal_orm::{Buildable, Field, SchemaGetter, SetterAssignable, ToRaw, E};

#[test]
fn test_setter() {
    let org = Organization::schema();

    org.users([3]).name.equal_to("Ye".to_string());
    // org.time().connected.equal_to("34");
    // org.users.name.equal_to(Time::default());
    org.age.equal_to(34);
    org.age.equal_to(Field::new("age"));
    let org = org.time().connected.equal_to(chrono::Utc::now());

    assert_eq!(org.fine_tune_params(), "time.connected = $_param_00000001");
    assert!(org.to_raw().build().len() > 40,);
}
