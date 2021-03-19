use std::sync::Arc;

use observable_tree::*;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() {
    let tree = BTree::start(1000);
    let btree = Arc::new(Mutex::new(tree));

    let btree_main = btree.clone();
    let m = btree_main.lock().await;
    let ins = (*m).insert("hello".to_string(), Types::Integer(5)).await;
    assert!(ins.is_none());

    let cont = (*m).contains("hello".to_string()).await;
    assert_eq!(cont, Some(Types::Boolean(true)));

    {
        let btree_async = btree.clone();
        tokio::spawn(async move {
            let t_m = btree_async.lock().await;
            let ins = (*t_m).insert("wow".to_string(), Types::Integer(5)).await;
            assert!(ins.is_none());

            let cont = (*t_m).contains("wow".to_string()).await;
            assert_eq!(cont, Some(Types::Boolean(true)));
        });
        println!("Done async 1");
    };

    {
        let btree_async2 = btree.clone();
        tokio::spawn(async move {
            let t_m = btree_async2.lock().await;

            let cont = (*t_m).contains("wow".to_string()).await;
            assert_eq!(cont, Some(Types::Boolean(true)));
            let cont = (*t_m).contains("hello".to_string()).await;
            assert_eq!(cont, Some(Types::Boolean(true)));
        });
        println!("Done async 2")
    };

    return;
}
