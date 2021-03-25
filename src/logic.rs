use crate::model::Types;

pub fn add(x: &mut Types, v: Types) -> Option<Types> {
    match (x.clone(), v) {
        (Types::Integer(xx), Types::Integer(vv)) => {
            *x = Types::Integer(xx + vv);
            Some(Types::Boolean(true))
        }
        (Types::UInteger(xx), Types::UInteger(vv)) => {
            *x = Types::UInteger(xx + vv);
            Some(Types::Boolean(true))
        }
        (Types::Float(xx), Types::Float(vv)) => {
            *x = Types::Float(xx + vv);
            Some(Types::Boolean(true))
        }
        (Types::String(xx), Types::String(vv)) => {
            *x = Types::String(xx + &vv);
            Some(Types::Boolean(true))
        }
        (Types::Vector(mut xx), Types::Vector(mut vv)) => {
            xx.append(&mut vv);
            *x = Types::Vector(xx);
            Some(Types::Boolean(true))
        }
        (Types::HashMap(mut xx), Types::KeyValue(k, vv)) => {
            let new_v = vv.as_ref();
            xx.insert(k, new_v.to_owned());
            *x = Types::HashMap(xx);
            Some(Types::Boolean(true))
        }
        (Types::BTreeMap(mut xx), Types::KeyValue(k, vv)) => {
            let new_v = vv.as_ref();
            xx.insert(k, new_v.to_owned());
            *x = Types::BTreeMap(xx);
            Some(Types::Boolean(true))
        }
        (Types::Vector(mut xx), vv) => {
            xx.push(vv);
            *x = Types::Vector(xx);
            Some(Types::Boolean(true))
        }
        _ => None,
    }
}
