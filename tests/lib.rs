use observable_btree::{model::Types, BTree};

#[tokio::test]
async fn test_insert_contains() {
    let btree = BTree::start(1000);

    let ins = btree.insert("hello".to_string(), 5).await;
    assert!(ins.unwrap().is_none());

    let cont = btree.contains("hello".to_string()).await;
    assert!(cont.unwrap());
}

#[tokio::test]
async fn test_insert_get() {
    let btree = BTree::start(1000);

    let ins = btree.insert("hello".to_string(), 5).await;
    assert!(ins.unwrap().is_none());

    let get = btree.get("hello".to_string()).await;
    let get_int = get.unwrap().unwrap();
    assert_eq!(get_int, Types::Integer(5));
}

#[tokio::test]
async fn test_insert_len() {
    let btree = BTree::start(1000);

    let ins = btree.insert("hello".to_string(), 5).await;
    assert!(ins.unwrap().is_none());

    let len = btree.len().await;
    assert_eq!(len.unwrap(), 1);
}

#[tokio::test]
async fn test_keys_values() {
    let btree = BTree::start(1000);

    let ins = btree.insert("hello".to_string(), 546).await;
    assert!(ins.unwrap().is_none());

    let ins = btree.insert("wow".to_string(), 5).await;
    assert!(ins.unwrap().is_none());

    let ins = btree.insert("what?".to_string(), 7).await;
    assert!(ins.unwrap().is_none());

    let ins = btree.insert("wow".to_string(), 15).await;
    assert_eq!(ins.unwrap(), Some(Types::Integer(5)));

    let cont = btree.keys().await;
    assert_eq!(
        cont.unwrap(),
        vec!["hello".to_string(), "what?".to_string(), "wow".to_string()]
    );

    let values = btree.values().await;
    assert_eq!(
        values.unwrap(),
        vec![Types::Integer(546), Types::Integer(7), Types::Integer(15)]
    );
}

#[tokio::test]
async fn test_remove() {
    let btree = BTree::start(1000);

    let ins = btree.insert("hello".to_string(), 5).await;
    assert!(ins.unwrap().is_none());

    let remove = btree.remove("hello".to_string()).await;
    let remove_int = remove.unwrap().unwrap();
    assert_eq!(remove_int, Types::Integer(5));
}

#[tokio::test]
async fn test_remove_entry() {
    let btree = BTree::start(1000);

    let ins = btree.insert("hello".to_string(), 5).await;
    assert!(ins.unwrap().is_none());

    let remove = btree.remove_entry("hello".to_string()).await;
    let remove_kv = remove.unwrap().unwrap();
    assert_eq!(
        remove_kv,
        Types::KeyValue("hello".to_string(), Box::new(Types::Integer(5)))
    );
}

#[tokio::test]
async fn test_insert_getmut() {
    let btree = BTree::start(1000);

    let ins = btree.insert("hello".to_string(), 5).await;
    assert!(ins.unwrap().is_none());

    let get_mut = btree
        .get_mut(
            "hello".to_string(),
            5,
            observable_btree::model::Operation::Add,
        )
        .await;
    assert!(get_mut.unwrap());

    let get = btree.get("hello".to_string()).await;
    let get_int = get.unwrap().unwrap();
    assert_eq!(get_int, Types::Integer(10));

    let get_mut = btree
        .get_mut(
            "hello".to_string(),
            4,
            observable_btree::model::Operation::Replace,
        )
        .await;
    assert!(get_mut.unwrap());

    let get = btree.get("hello".to_string()).await;
    let get_int = get.unwrap().unwrap();
    assert_eq!(get_int, Types::Integer(4));
}
