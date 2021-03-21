use std::{collections::BTreeMap, convert::TryInto};

use tokio::sync::mpsc::{self, Sender};
use tokio::sync::oneshot;

pub mod model;

use model::Types;

pub enum Action {
    Insert(String, Types),
    Contains(String),
    Get(String),
    Len,
    Keys,
    Values,
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
                            println!("the receiver dropped, mpsc get k: {}", k);
                        }
                    }
                    Action::Keys => {
                        let get = btree.keys();
                        let keys: Vec<Types> = get.map(|k| k.to_owned().into()).collect();

                        if let Err(_) = tx_o.send(Some(Types::Vector(keys))) {
                            println!("the receiver dropped, mpsc get keys");
                        }
                    }
                    Action::Values => {
                        let get = btree.values();
                        let values: Vec<Types> = get.map(|k| k.to_owned()).collect();

                        if let Err(_) = tx_o.send(Some(Types::Vector(values))) {
                            println!("the receiver dropped, mpsc get values");
                        }
                    }
                    Action::Len => {
                        let len = btree.len();
                        if let Err(_) = tx_o.send(Some(Types::UInteger(len))) {
                            println!("the receiver dropped, mpsc len");
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

    pub async fn len(&self) -> Result<usize, String> {
        let tx = self.tx.clone();
        let (tx_o, rx_o) = oneshot::channel();
        let action = Action::Len;
        let send = (action, tx_o);

        tx.send(send)
            .await
            .map_err(|_| format!("receiver dropped, len",))?;

        match rx_o.await {
            Ok(Some(Types::UInteger(len))) => Ok(len),
            _ => Err(format!("len failed")),
        }
    }

    pub async fn keys(&self) -> Result<Vec<String>, String> {
        let tx = self.tx.clone();
        let (tx_o, rx_o) = oneshot::channel();
        let action = Action::Keys;
        let send = (action, tx_o);

        tx.send(send)
            .await
            .map_err(|_| format!("receiver dropped, get keys"))?;

        match rx_o.await {
            Ok(Some(Types::Vector(types))) => {
                let vec = types
                    .into_iter()
                    .map(|k| k.try_into())
                    .collect::<Result<Vec<String>, String>>();
                vec
            }
            Err(e) => Err(format!("get keys failed with error: {:?}", e)),
            _ => Err(format!("get keys failed")),
        }
    }

    pub async fn values(&self) -> Result<Vec<Types>, String> {
        let tx = self.tx.clone();
        let (tx_o, rx_o) = oneshot::channel();
        let action = Action::Values;
        let send = (action, tx_o);

        tx.send(send)
            .await
            .map_err(|_| format!("receiver dropped, get values"))?;

        match rx_o.await {
            Ok(Some(Types::Vector(types))) => Ok(types),
            Err(e) => Err(format!("get values failed with error: {:?}", e)),
            _ => Err(format!("get values failed")),
        }
    }
}
