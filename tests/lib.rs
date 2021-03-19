use observable_tree::*;

#[tokio::test]
async fn test_insert_contains() {
    let btree = BTree::start(1000);

    let ins = btree.insert("hello".to_string(), Types::Integer(5)).await;
    assert!(ins.is_none());

    let cont = btree.contains("hello".to_string()).await;
    assert_eq!(cont, Some(Types::Boolean(true)));
}
