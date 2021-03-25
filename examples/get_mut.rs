use observable_tree::{model::Types, BTree};

#[tokio::main]
async fn main() {
    let btree = BTree::start(1000);

    let ins = btree.insert("hello".to_string(), 5).await;
    assert!(ins.unwrap().is_none());

    let get_mut = btree
        .get_mut(
            "hello".to_string(),
            5,
            observable_tree::model::Operation::Add,
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
            observable_tree::model::Operation::Replace,
        )
        .await;
    assert!(get_mut.unwrap());

    let get = btree.get("hello".to_string()).await;
    let get_int = get.unwrap().unwrap();
    assert_eq!(get_int, Types::Integer(4));

    print!("Done!")
}
