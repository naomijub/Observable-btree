use observable_tree::{model::Types, BTree};

#[tokio::main]
async fn main() {
    let btree = BTree::start(1000);

    let ins = btree.insert("hello".to_string(), 546).await;
    assert!(ins.unwrap().is_none());

    let ins = btree.insert("wow".to_string(), 5).await;
    assert!(ins.unwrap().is_none());

    let ins = btree.insert("what?".to_string(), 7).await;
    assert!(ins.unwrap().is_none());

    // Remember it is a BTreeMap so values come back ordered
    let keys = btree.keys().await;
    assert_eq!(
        keys.unwrap(),
        vec!["hello".to_string(), "what?".to_string(), "wow".to_string()]
    );

    let values = btree.values().await;
    assert_eq!(
        values.unwrap(),
        vec![Types::Integer(546), Types::Integer(7), Types::Integer(5)]
    );

    return;
}
