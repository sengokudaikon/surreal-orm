/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowo.oss@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

use surreal_orm::migrator::{self, embed_migrations};

const _IGRATIONS_ONEWAY: migrator::EmbeddedMigrationsOneWay =
    embed_migrations!("../migrator/oneway", strict);
const _MIGRATIONS_ONEWAY: migrator::EmbeddedMigrationsOneWay =
    embed_migrations!("../migrator/oneway", strict);

// const MIGRATIONS2: migrator::EmbeddedMigrationsTwoWay = embed_migrations!("../migrator/migrations");
// const MIGRATIONS2: migrator::EmbeddedMigrationsTwoWay =
const _MIGRATIONS2_TWOWAY: migrator::EmbeddedMigrationsTwoWay =
    embed_migrations!("../examples/migration-embedded/migrations");

#[test]
fn test_embed_migrations() {
    // insta::assert_snapshot!(MIGRATIONS.migrations.to_vec());
}
