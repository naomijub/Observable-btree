use observable_tree::{model::Types, BTree};

#[tokio::main]
async fn main() {
    let btree = BTree::start(1000);

    let ins = btree.insert("hello".to_string(), 546).await;
    assert!(ins.unwrap().is_none());

    let get = btree.get("hello".to_string()).await;
    let get_int = get.unwrap().unwrap();
    assert_eq!(get_int, Types::Integer(5));

    return;
}
