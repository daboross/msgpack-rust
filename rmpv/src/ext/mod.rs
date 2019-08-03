use std::{
    error,
    fmt::{self, Display, Formatter},
};

use serde::de::Unexpected;

use IntPriv;
use Integer;
use Value;
use ValueRef;

pub use self::{
    de::{deserialize_from, from_value, EnumRefDeserializer},
    se::to_value,
};

mod de;
mod se;

#[derive(Debug)]
pub enum Error {
    Syntax(String),
}

impl Display for Error {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), fmt::Error> {
        match *self {
            Error::Syntax(ref err) => write!(fmt, "{}: {}", error::Error::description(self), err),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        "error while decoding value"
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            Error::Syntax(..) => None,
        }
    }
}

trait ValueExt {
    fn unexpected(&self) -> Unexpected;
}

impl ValueExt for Value {
    fn unexpected(&self) -> Unexpected {
        match *self {
            Value::Nil => Unexpected::Unit,
            Value::Boolean(v) => Unexpected::Bool(v),
            Value::Integer(Integer { n }) => match n {
                IntPriv::PosInt(v) => Unexpected::Unsigned(v),
                IntPriv::NegInt(v) => Unexpected::Signed(v),
            },
            Value::F32(v) => Unexpected::Float(v as f64),
            Value::F64(v) => Unexpected::Float(v),
            Value::String(ref v) => match v.s {
                Ok(ref v) => Unexpected::Str(v),
                Err(ref v) => Unexpected::Bytes(&v.0[..]),
            },
            Value::Binary(ref v) => Unexpected::Bytes(v),
            Value::Array(..) => Unexpected::Seq,
            Value::Map(..) => Unexpected::Map,
            Value::Ext(..) => Unexpected::Seq,
        }
    }
}

impl<'a> ValueExt for ValueRef<'a> {
    fn unexpected(&self) -> Unexpected {
        match *self {
            ValueRef::Nil => Unexpected::Unit,
            ValueRef::Boolean(v) => Unexpected::Bool(v),
            ValueRef::Integer(Integer { n }) => match n {
                IntPriv::PosInt(v) => Unexpected::Unsigned(v),
                IntPriv::NegInt(v) => Unexpected::Signed(v),
            },
            ValueRef::F32(v) => Unexpected::Float(v as f64),
            ValueRef::F64(v) => Unexpected::Float(v),
            ValueRef::String(ref v) => match v.s {
                Ok(ref v) => Unexpected::Str(v),
                Err(ref v) => Unexpected::Bytes(&v.0[..]),
            },
            ValueRef::Binary(ref v) => Unexpected::Bytes(v),
            ValueRef::Array(..) => Unexpected::Seq,
            ValueRef::Map(..) => Unexpected::Map,
            ValueRef::Ext(..) => Unexpected::Seq,
        }
    }
}
