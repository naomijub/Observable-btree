#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

// use tokio::sync::mpsc;

// #[tokio::main]
// async fn main() {
//     let (tx, mut rx) = mpsc::channel(100);

//     tokio::spawn(async move {
//         for i in 0..10 {
//             if let Err(_) = tx.send(i).await {
//                 println!("receiver dropped");
//                 return;
//             }
//         }
//     });

//     while let Some(i) = rx.recv().await {
//         println!("got = {}", i);
//     }
// }

// use tokio::sync::oneshot;

// #[tokio::main]
// async fn main() {
//     let (tx, rx) = oneshot::channel();

//     tokio::spawn(async move {
//         if let Err(_) = tx.send(3) {
//             println!("the receiver dropped");
//         }
//     });

//     match rx.await {
//         Ok(v) => println!("got = {:?}", v),
//         Err(_) => println!("the sender dropped"),
//     }
// }