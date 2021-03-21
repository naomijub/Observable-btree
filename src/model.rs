use std::{
    collections::{BTreeMap, HashMap},
    convert::{TryFrom, TryInto},
};

/// Available types to use as `BTree` values.
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

impl From<char> for Types {
    fn from(t: char) -> Self {
        Types::Char(t)
    }
}

impl From<isize> for Types {
    fn from(t: isize) -> Self {
        Types::Integer(t)
    }
}

impl From<i32> for Types {
    fn from(t: i32) -> Self {
        Types::Integer(t as isize)
    }
}

impl From<i64> for Types {
    fn from(t: i64) -> Self {
        Types::Integer(t as isize)
    }
}

impl From<usize> for Types {
    fn from(t: usize) -> Self {
        Types::UInteger(t)
    }
}

impl From<String> for Types {
    fn from(t: String) -> Self {
        Types::String(t)
    }
}

impl From<&str> for Types {
    fn from(t: &str) -> Self {
        Types::String(t.to_owned())
    }
}

impl From<f64> for Types {
    fn from(t: f64) -> Self {
        Types::Float(t)
    }
}

impl From<bool> for Types {
    fn from(t: bool) -> Self {
        Types::Boolean(t)
    }
}

impl<T: Into<Types>> From<Vec<T>> for Types {
    fn from(t: Vec<T>) -> Self {
        let aux = t.into_iter().map(|e| e.into()).collect::<Vec<Types>>();
        Types::Vector(aux)
    }
}

impl<T: Into<Types>> From<HashMap<String, T>> for Types {
    fn from(t: HashMap<String, T>) -> Self {
        let aux = t
            .into_iter()
            .map(|(k, v)| (k, v.into()))
            .collect::<HashMap<String, Types>>();
        Types::HashMap(aux)
    }
}

impl<T: Into<Types>> From<BTreeMap<String, T>> for Types {
    fn from(t: BTreeMap<String, T>) -> Self {
        let aux = t
            .into_iter()
            .map(|(k, v)| (k, v.into()))
            .collect::<BTreeMap<String, Types>>();
        Types::BTreeMap(aux)
    }
}

impl<T: Into<Types>> From<(String, T)> for Types {
    fn from(t: (String, T)) -> Self {
        let (k, v) = t;
        let (k, v): (String, Box<Types>) = (k, Box::new(v.into()));

        Types::KeyValue(k, v)
    }
}

impl<T: Into<Types>> From<Option<T>> for Types {
    fn from(t: Option<T>) -> Self {
        match t {
            None => Types::Nil,
            Some(v) => v.into(),
        }
    }
}

impl TryInto<isize> for Types {
    type Error = String;

    fn try_into(self) -> Result<isize, Self::Error> {
        match self {
            Types::Integer(t) => Ok(t),
            _ => Err(format!("Could not convert {:?} to isize", self)),
        }
    }
}

impl TryInto<usize> for Types {
    type Error = String;

    fn try_into(self) -> Result<usize, Self::Error> {
        match self {
            Types::UInteger(t) => Ok(t),
            _ => Err(format!("Could not convert {:?} to usize", self)),
        }
    }
}

impl TryInto<char> for Types {
    type Error = String;

    fn try_into(self) -> Result<char, Self::Error> {
        match self {
            Types::Char(t) => Ok(t),
            _ => Err(format!("Could not convert {:?} to char", self)),
        }
    }
}

impl TryInto<f64> for Types {
    type Error = String;

    fn try_into(self) -> Result<f64, Self::Error> {
        match self {
            Types::Float(t) => Ok(t),
            _ => Err(format!("Could not convert {:?} to f64", self)),
        }
    }
}

impl TryInto<bool> for Types {
    type Error = String;

    fn try_into(self) -> Result<bool, Self::Error> {
        match self {
            Types::Boolean(t) => Ok(t),
            _ => Err(format!("Could not convert {:?} to bool", self)),
        }
    }
}

impl TryInto<String> for Types {
    type Error = String;

    fn try_into(self) -> Result<String, Self::Error> {
        match self {
            Types::String(t) => Ok(t),
            _ => Err(format!("Could not convert {:?} to String", self)),
        }
    }
}

impl<T: TryFrom<Types>> TryInto<(String, T)> for Types {
    type Error = String;

    fn try_into(self) -> Result<(String, T), Self::Error> {
        let err = format!("Could not convert {:?} to KeyValue", self);
        match self {
            Types::KeyValue(k, v) => Ok((k, (*v).try_into().map_err(|_| err.clone())?)),
            _ => Err(format!("Could not convert {:?} to KeyValue", self)),
        }
    }
}

impl<T: TryFrom<Types>> TryInto<Vec<T>> for Types {
    type Error = String;

    fn try_into(self) -> Result<Vec<T>, Self::Error> {
        let err = format!("Could not convert {:?} to Vec<T>", self);
        match self {
            Types::Vector(t) => t
                .into_iter()
                .map(|e| e.try_into().map_err(|_| err.clone()))
                .collect::<Result<Vec<T>, String>>(),
            _ => Err(err),
        }
    }
}

impl<T: TryFrom<Types>> TryInto<HashMap<String, T>> for Types {
    type Error = String;

    fn try_into(self) -> Result<HashMap<String, T>, Self::Error> {
        let err = format!("Could not convert {:?} to HashMap<String, T>", self);
        match self {
            Types::HashMap(t) => {
                let mut has_error = false;
                let hm = t
                    .into_iter()
                    .map(|(k, v)| (k, v.try_into().map_err(|_| err.clone())))
                    .fold(
                        HashMap::new(),
                        |mut acc, (k, v): (String, Result<T, String>)| {
                            if let Ok(t) = v {
                                acc.insert(k, t);
                            } else {
                                has_error = true;
                            }
                            acc
                        },
                    );

                if has_error {
                    Err(err)
                } else {
                    Ok(hm)
                }
            }
            _ => Err(err),
        }
    }
}

impl<T: TryFrom<Types>> TryInto<BTreeMap<String, T>> for Types {
    type Error = String;

    fn try_into(self) -> Result<BTreeMap<String, T>, Self::Error> {
        let err = format!("Could not convert {:?} to BTreeMap<String, T>", self);
        match self {
            Types::BTreeMap(t) => {
                let mut has_error = false;
                let hm = t
                    .into_iter()
                    .map(|(k, v)| (k, v.try_into().map_err(|_| err.clone())))
                    .fold(
                        BTreeMap::new(),
                        |mut acc, (k, v): (String, Result<T, String>)| {
                            if let Ok(t) = v {
                                acc.insert(k, t);
                            } else {
                                has_error = true;
                            }
                            acc
                        },
                    );

                if has_error {
                    Err(err)
                } else {
                    Ok(hm)
                }
            }
            _ => Err(err),
        }
    }
}
