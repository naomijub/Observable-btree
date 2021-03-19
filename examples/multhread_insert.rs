use std::sync::Arc;
use tokio::sync::Mutex;

use observable_tree::BTree;

#[tokio::main]
async fn main() {
    let tree = BTree::start(1000);
    let btree = Arc::new(Mutex::new(tree));

    let btree_main = btree.clone();
    let m = btree_main.lock().await;
    let ins = (*m).insert("hello".to_string(), 5).await;
    assert!(ins.unwrap().is_none());

    let cont = (*m).contains("hello".to_string()).await;
    assert!(cont.unwrap());

    {
        let btree_async = btree.clone();
        tokio::spawn(async move {
            let t_m = btree_async.lock().await;
            let ins = (*t_m).insert("wow".to_string(), 76).await;
            assert!(ins.unwrap().is_none());

            let cont = (*t_m).contains("wow".to_string()).await;
            assert!(cont.unwrap());
        });
        println!("Done async 1");
    };

    {
        let btree_async2 = btree.clone();
        tokio::spawn(async move {
            let t_m = btree_async2.lock().await;

            let cont = (*t_m).contains("wow".to_string()).await;
            assert!(cont.unwrap());
            let cont = (*t_m).contains("hello".to_string()).await;
            assert!(cont.unwrap());
        });
        println!("Done async 2")
    };

    return;
}
