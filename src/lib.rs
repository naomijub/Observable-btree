use std::collections::{BTreeMap, HashMap};

use tokio::sync::mpsc::{self, Sender};
use tokio::sync::oneshot;

#[derive(Debug, Clone, PartialEq)]
pub enum Types {
    Char(char),
    Integer(isize),
    UInteger(usize),
    String(String),
    Float(f64),
    Boolean(bool),
    Vector(Vec<Types>),
    HashMap(HashMap<String, Types>),
    BTreeMap(BTreeMap<String, Types>),
    KeyValue(String, Box<Types>),
    Nil,
}

pub enum Action {
    Insert(String, Types),
    Contains(String),
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
                }
            }
        });

        Self { tx }
    }

    pub async fn insert(&self, k: String, v: Types) -> Option<Types> {
        let tx = self.tx.clone();
        let (tx_o, rx_o) = oneshot::channel();
        let action = Action::Insert(k.clone(), v.clone());
        let send = (action, tx_o);

        if let Err(_) = tx.send(send).await {
            println!("receiver dropped, insert k: {}, v: {:?}", k, v);
        }

        match rx_o.await {
            Ok(v) => v,
            Err(_) => Some(Types::Nil),
        }
    }

    pub async fn contains(&self, k: String) -> Result<bool, String> {
        let tx = self.tx.clone();
        let (tx_o, rx_o) = oneshot::channel();
        let action = Action::Contains(k.clone());
        let send = (action, tx_o);

        if let Err(_) = tx.send(send).await {
            println!("receiver dropped, contains {}", k);
        }

        match rx_o.await {
            Ok(Some(Types::Boolean(true))) => Ok(true),
            Err(e) => Err(format!("{:?}", e)),
            _ => Ok(false),
        }
    }
}
