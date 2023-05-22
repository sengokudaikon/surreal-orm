use surrealdb::{engine::local::Mem, Surreal};
use surrealdb_models::{Account, Balance};
use surrealdb_orm::{
    statements::{begin_transaction, create, select, update},
    *,
};
// CREATE account:one SET balance = 135,605.16;
// CREATE account:two SET balance = 91,031.31;
// -- Move money
// UPDATE account:one SET balance += 300.00;
// UPDATE account:two SET balance -= 300.00;
// test Increment update and decrement update
#[tokio::test]
async fn test_transaction_commit_increment_and_decrement_update() -> SurrealdbOrmResult<()> {
    let db = Surreal::new::<Mem>(()).await.unwrap();
    db.use_ns("test").use_db("test").await.unwrap();

    let ref id1 = Account::create_id("one".into());
    let ref id2 = Account::create_id("two".into());
    let amount_to_transfer = 300.00;

    let acc = Account::schema();

    begin_transaction()
        .query(create(Account {
            id: id1.clone(),
            balance: 135_605.16,
        }))
        .query(create(Account {
            id: id2.clone(),
            balance: 91_031.31,
        }))
        .query(update::<Account>(id1).set(acc.balance.increment_by(amount_to_transfer)))
        .query(update::<Account>(id2).set(acc.balance.decrement_by(amount_to_transfer)))
        .commit_transaction()
        .run(db.clone())
        .await?;

    let accounts = select(All)
        .from(id1..=id2)
        .return_many::<Account>(db.clone())
        .await?;

    assert_eq!(accounts.len(), 2);
    assert_eq!(accounts[0].balance, 135_905.16);
    assert_eq!(accounts[1].balance, 90_731.31);
    assert_eq!(accounts[0].id.to_string(), "account:one");
    assert_eq!(accounts[1].id.to_string(), "account:two");

    Ok(())
}

#[tokio::test]
async fn test_transaction_cancellation_increment_and_decrement_update() -> SurrealdbOrmResult<()> {
    let db = Surreal::new::<Mem>(()).await.unwrap();
    db.use_ns("test").use_db("test").await.unwrap();

    let ref id1 = Account::create_id("one".into());
    let ref id2 = Account::create_id("two".into());

    let acc = Account::schema();

    begin_transaction()
        .query(create(Account {
            id: id1.clone(),
            balance: 135_605.16,
        }))
        .query(create(Account {
            id: id2.clone(),
            balance: 91_031.31,
        }))
        .query(update::<Account>(id1).set(acc.balance.increment_by(300.00)))
        .query(update::<Account>(id2).set(acc.balance.decrement_by(300.00)))
        .cancel_transaction()
        .run(db.clone())
        .await?;

    let accounts = select(All)
        .from(id1..=id2)
        .return_many::<Account>(db.clone())
        .await?;

    assert_eq!(accounts.len(), 0);
    Ok(())
}

#[tokio::test]
async fn test_transaction_with_surreal_queries_macro() -> SurrealdbOrmResult<()> {
    let db = Surreal::new::<Mem>(()).await.unwrap();
    db.use_ns("test").use_db("test").await.unwrap();

    let ref id1 = Account::create_id("one".into());
    let ref id2 = Account::create_id("two".into());
    let acc = Account::schema();

    let amount_to_transfer = 300.00;
    begin_transaction()
        .query(queries!(
            let balance = create(Balance {
                balance: amount_to_transfer,
                ..Default::default()
            });

            create(Account {
                id: id1.clone(),
                balance: 135_605.16,
            });

            create(Account {
                id: id2.clone(),
                balance: 91_031.31,
            });

            update::<Account>(id1).set(acc.balance.increment_by(balance.with_path::<Balance>(E).balance));
            update::<Account>(id2).set(acc.balance.decrement_by(amount_to_transfer));
        ))
        .commit_transaction()
        .run(db.clone())
        .await?;

    let accounts = select(All)
        .from(id1..=id2)
        .return_many::<Account>(db.clone())
        .await?;

    assert_eq!(accounts.len(), 2);
    assert_eq!(accounts[0].balance, 135_905.16);
    assert_eq!(accounts[1].balance, 90_731.31);
    assert_eq!(accounts[0].id.to_string(), "account:one");
    assert_eq!(accounts[1].id.to_string(), "account:two");

    Ok(())
}
#[tokio::test]
async fn test_transaction_with_block_macro() -> SurrealdbOrmResult<()> {
    let db = Surreal::new::<Mem>(()).await.unwrap();
    db.use_ns("test").use_db("test").await.unwrap();

    let ref id1 = Account::create_id("one".into());
    let ref id2 = Account::create_id("two".into());
    let amount_to_transfer = 300.00;

    let acc = Account::schema();

    block! {
        BEGIN TRANSACTION;

        let acc1 = create(Account {
            id: id1.clone(),
            balance: 135_605.16,
        });
        let acc2 = create(Account {
            id: id2.clone(),
            balance: 91_031.31,
        });

        let updated1 = update::<Account>(id1).set(acc.balance.increment_by(amount_to_transfer));
        let update2 = update::<Account>(id2).set(acc.balance.decrement_by(amount_to_transfer));

        COMMIT TRANSACTION;
    }
    .run(db.clone())
    .await?;

    let accounts = select(All)
        .from(id1..=id2)
        .return_many::<Account>(db.clone())
        .await?;

    assert_eq!(accounts.len(), 2);
    assert_eq!(accounts[0].balance, 135_905.16);
    assert_eq!(accounts[1].balance, 90_731.31);
    assert_eq!(accounts[0].id.to_string(), "account:one");
    assert_eq!(accounts[1].id.to_string(), "account:two");

    Ok(())
}
