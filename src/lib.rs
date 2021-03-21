use std::{collections::BTreeMap, convert::TryInto};

use tokio::sync::mpsc::{self, Sender};
use tokio::sync::oneshot;

pub mod model;

use model::Types;

enum Action {
    Insert(String, Types),
    Contains(String),
    Get(String),
    Len,
    Keys,
    Values,
    Remove(String),
    RemoveEntry(String),
}

/// `BTree` is where the informatio `Sender` is contained.
/// Its inner implementation has a tuple containing the action to be taken as well as a oneshot channel to receive data.
/// To start the `BTree` thread just execute `BTree::start(buffer_size: usize)`. If you `buffer_size` is too short
/// it may cause synchronization problems, so it should be well ajusted to your application needs.
pub struct BTree {
    tx: Sender<(Action, tokio::sync::oneshot::Sender<Option<Types>>)>,
}

impl BTree {
    /// `BTree::start(buffer_size: usize)` is the entrypoint to start using `BTree` methods.
    /// It creates a thread containing the BTreeMap and keeps listening to entries.
    pub fn start(buffer_size: usize) -> Self {
        let (tx, mut rx) = mpsc::channel(buffer_size);
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
                    Action::Remove(k) => {
                        let remove = btree.remove(&k);

                        if let Err(_) = tx_o.send(remove) {
                            println!("the receiver dropped, mpsc remove for key: {}", k);
                        }
                    }
                    Action::RemoveEntry(k) => {
                        let remove = btree.remove_entry(&k);
                        let key_val = if let Some((key, value)) = remove {
                            Some(Types::KeyValue(key, Box::new(value)))
                        } else {
                            None
                        };
                        if let Err(_) = tx_o.send(key_val) {
                            println!("the receiver dropped, mpsc remove_entry for key: {}", k);
                        }
                    }
                }
            }
        });

        Self { tx }
    }

    /// Method `insert` is equivalent to [`std::collection::BTreeMap insert`](https://doc.rust-lang.org/std/collections/struct.BTreeMap.html#method.insert),
    /// it returns `None` if the key does not exist and it returns `Some(Types::_)` with the previous value,
    /// if the key already exists.
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

    /// Method `contains` is equivalent to [`std::collection::BTreeMap contains_key`](https://doc.rust-lang.org/std/collections/struct.BTreeMap.html#method.contains_key),
    /// It checks if a key already exists in the `BTree`. If the key exists the return is `Ok(true)`,
    /// if it doesn't exist it returns `Ok(false)`
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

    /// Method `get` is equivalent to [`std::collection::BTreeMap get`](https://doc.rust-lang.org/std/collections/struct.BTreeMap.html#method.get),
    /// It returns the value contained at the key passed as argument. If no key is found the return is `Ok(None)`,
    /// else it returns `Ok(Some(Types::_))`.
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

    /// Method `len` is equivalent to [`std::collection::BTreeMap len`](https://doc.rust-lang.org/std/collections/struct.BTreeMap.html#method.len),
    /// It returns the length of the btree as a usize.
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

    /// Method `keys` is equivalent to [`std::collection::BTreeMap keys`](https://doc.rust-lang.org/std/collections/struct.BTreeMap.html#method.keys),
    /// It returns a vector containing all the keys sorted.
    /// For `BTree` the keys are always `Strings`.
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

    /// Method `values` is equivalent to [`std::collection::BTreeMap values`](https://doc.rust-lang.org/std/collections/struct.BTreeMap.html#method.values),
    /// It returns a vector containing all the values sorted by their respective keys order.
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

    /// Method `remove` is equivalent to [`std::collection::BTreeMap remove`](https://doc.rust-lang.org/std/collections/struct.BTreeMap.html#method.remove),
    /// It returns the value removed from the `BTree` for the key passed as argument. If no key is found the return is `Ok(None)`,
    /// else it returns `Ok(Some(Types::_))`.
    pub async fn remove(&self, k: String) -> Result<Option<Types>, String> {
        let tx = self.tx.clone();
        let (tx_o, rx_o) = oneshot::channel();
        let action = Action::Remove(k.clone());
        let send = (action, tx_o);

        tx.send(send)
            .await
            .map_err(|_| format!("receiver dropped, remove key {}", k))?;

        match rx_o.await {
            Ok(types) => Ok(types),
            Err(e) => Err(format!("remove failed {} with error: {:?}", k, e)),
        }
    }

    /// Method `remove_entry` is equivalent to [`std::collection::BTreeMap remove_entry`](https://doc.rust-lang.org/std/collections/struct.BTreeMap.html#method.remove_entry),
    /// It returns the key and value removed from the `BTree` for the key passed as argument as a `Option<Types::KeyValue(_,_)>`.
    /// If no key is found the return is `Ok(None)`,
    pub async fn remove_entry(&self, k: String) -> Result<Option<Types>, String> {
        let tx = self.tx.clone();
        let (tx_o, rx_o) = oneshot::channel();
        let action = Action::RemoveEntry(k.clone());
        let send = (action, tx_o);

        tx.send(send)
            .await
            .map_err(|_| format!("receiver dropped, remove_entry key {}", k))?;

        match rx_o.await {
            Ok(types) => Ok(types),
            Err(e) => Err(format!("remove_entry failed {} with error: {:?}", k, e)),
        }
    }
}
