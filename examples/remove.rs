use observable_tree::{model::Types, BTree};

#[tokio::main]
async fn main() {
    let btree = BTree::start(1000);

    let ins = btree.insert("hello".to_string(), 546).await;
    assert!(ins.unwrap().is_none());
    let ins = btree.insert("wow".to_string(), 5).await;
    assert!(ins.unwrap().is_none());

    let remove = btree.remove("hello".to_string()).await;
    let remove_int = remove.unwrap().unwrap();
    assert_eq!(remove_int, Types::Integer(546));

    let remove = btree.remove_entry("wow".to_string()).await;
    let remove_kv = remove.unwrap().unwrap();
    assert_eq!(
        remove_kv,
        Types::KeyValue("wow".to_string(), Box::new(Types::Integer(5)))
    );

    print!("Done!")
}
