use observable_tree::{model::Types, BTree};

#[tokio::main]
async fn main() {
    let btree = BTree::start(1000);

    let ins = btree.insert("hello".to_string(), Types::Integer(5)).await;
    assert!(ins.unwrap().is_none());

    let cont = btree.contains("hello".to_string()).await;
    assert!(cont.unwrap());

    return;
}
