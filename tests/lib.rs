use observable_tree::{model::Types, BTree};

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
