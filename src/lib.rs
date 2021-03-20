use std::collections::BTreeMap;

use tokio::sync::mpsc::{self, Sender};
use tokio::sync::oneshot;

pub mod model;

use model::Types;

pub enum Action {
    Insert(String, Types),
    Contains(String),
    Get(String),
}

pub struct BTree {
    tx: Sender<(Action, tokio::sync::oneshot::Sender<Option<Types>>)>,
}

impl BTree {
    pub fn start(buffer: usize) -> Self {
        let (tx, mut rx) = mpsc::channel(buffer);
        tokio::spawn(async move {
            let mut btree: BTreeMap<String, Types> = BTreeMap::new();
            while let Some((action, tx_o)) = rx.recv().await {
                let tx_o: tokio::sync::oneshot::Sender<Option<Types>> = tx_o;
                match action {
                    Action::Insert(k, v) => {
                        let insert = btree.insert(k, v);
                        if let Err(_) = tx_o.send(insert) {
                            println!("the receiver dropped, mpsc insert");
                        }
                    }
                    Action::Contains(k) => {
                        let contains = btree.contains_key(&k);
                        if let Err(_) = tx_o.send(Some(Types::Boolean(contains))) {
                            println!("the receiver dropped, mpsc contains k: {}", k);
                        }
                    }
                    Action::Get(k) => {
                        let get = btree.get(&k);
                        let get = if let Some(types) = get {
                            Some(types.to_owned())
                        } else {
                            None
                        };
                        if let Err(_) = tx_o.send(get) {
                            println!("the receiver dropped, mpsc contains k: {}", k);
                        }
                    }
                }
            }
        });

        Self { tx }
    }

    pub async fn insert<V: Into<Types>>(&self, k: String, v: V) -> Result<Option<Types>, String> {
        let v = v.into();
        let tx = self.tx.clone();
        let (tx_o, rx_o) = oneshot::channel();
        let action = Action::Insert(k.clone(), v.clone());
        let send = (action, tx_o);

        tx.send(send)
            .await
            .map_err(|_| format!("receiver dropped, insert key {}, value {:?}", k, v))?;

        rx_o.await
            .map_err(|_| format!("insert failed {}, value {:?}", k, v))
    }

    pub async fn contains(&self, k: String) -> Result<bool, String> {
        let tx = self.tx.clone();
        let (tx_o, rx_o) = oneshot::channel();
        let action = Action::Contains(k.clone());
        let send = (action, tx_o);

        tx.send(send)
            .await
            .map_err(|_| format!("receiver dropped, contains key {}", k))?;
        match rx_o.await {
            Ok(Some(Types::Boolean(true))) => Ok(true),
            Err(e) => Err(format!("{:?}", e)),
            _ => Ok(false),
        }
    }

    pub async fn get(&self, k: String) -> Result<Option<Types>, String> {
        let tx = self.tx.clone();
        let (tx_o, rx_o) = oneshot::channel();
        let action = Action::Get(k.clone());
        let send = (action, tx_o);

        tx.send(send)
            .await
            .map_err(|_| format!("receiver dropped, get key {}", k))?;

        match rx_o.await {
            Ok(types) => Ok(types),
            Err(e) => Err(format!("get failed {} with error: {:?}", k, e)),
        }
    }
}
