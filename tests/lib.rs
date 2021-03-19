use observable_tree::{model::Types, BTree};

#[tokio::test]
async fn test_insert_contains() {
    let btree = BTree::start(1000);

    let ins = btree.insert("hello".to_string(), Types::Integer(5)).await;
    assert!(ins.unwrap().is_none());

    let cont = btree.contains("hello".to_string()).await;
    assert!(cont.unwrap());
}
