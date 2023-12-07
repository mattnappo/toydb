use super::super::super::{
    error::Error,
    types::{
        db::DbName,
        table::{LabeledTypedTuple, TableName, Value},
    },
};
use actix::Message;
use serde::{Deserialize, Serialize};

/// A message issued to the engine requesting that a tuple be inserted into the table.
#[derive(Message, Debug)]
#[rtype(result = "Result<(), Error>")]
pub struct Insert {
    pub(crate) db_name: DbName,
    pub(crate) table_name: TableName,
    pub(crate) values: Vec<Vec<u8>>,
}

/// A comparison between two comparators
#[derive(Serialize, Deserialize, Debug)]
pub enum Cmp {
    Eq(Comparator, Comparator),
    Lt(Comparator, Comparator),
    Gt(Comparator, Comparator),
    Ne(Comparator, Comparator),
}

impl Cmp {
    /// Returns whether or not the value falls under the comparison clause.
    pub fn has_value(&self, tup: &LabeledTypedTuple, attr_names: impl AsRef<[String]>) -> bool {
        self.helper(tup, attr_names).unwrap_or_default()
    }

    fn helper(&self, tup: &LabeledTypedTuple, attr_names: impl AsRef<[String]>) -> Option<bool> {
        match self {
            Self::Eq(a, b) => {
                let a_val = a.to_value_for(tup, attr_names.as_ref())?;
                let b_val = b.to_value_for(tup, attr_names.as_ref())?;

                Some(a_val == b_val)
            }
            Self::Lt(a, b) => {
                let a_val = a.to_value_for(tup, attr_names.as_ref())?;
                let b_val = b.to_value_for(tup, attr_names.as_ref())?;

                Some(a_val < b_val)
            }
            Self::Gt(a, b) => {
                let a_val = a.to_value_for(tup, attr_names.as_ref())?;
                let b_val = b.to_value_for(tup, attr_names.as_ref())?;

                Some(a_val > b_val)
            }
            Self::Ne(a, b) => {
                let a_val = a.to_value_for(tup, attr_names.as_ref())?;
                let b_val = b.to_value_for(tup, attr_names.as_ref())?;

                Some(a_val != b_val)
            }
        }
    }
}

/// Items that can be compared.
#[derive(Serialize, Deserialize, Debug)]
pub enum Comparator {
    Col(String),
    Val(Value),
}

impl Comparator {
    fn to_value_for(
        &self,
        tup: &LabeledTypedTuple,
        attr_names: impl AsRef<[String]>,
    ) -> Option<Value> {
        match self {
            Self::Col(c) => tup
                .0
                .clone()
                .into_iter()
                .zip(attr_names.as_ref().iter())
                .find(|(_, name)| name.as_str() == c.as_str())
                .map(|(val, _)| val.1),
            Self::Val(v) => Some(v.clone()),
        }
    }
}

/// A message issued to the engine requesting that a table be retrieved.
#[derive(Message, Debug)]
#[rtype(result = "Result<Vec<LabeledTypedTuple>, Error>")]
pub struct Select {
    pub(crate) db_name: DbName,
    pub(crate) table_name: TableName,
    pub(crate) filter: Option<Cmp>,
}

/// A message issued to the engine requesting that specific columns in a table be retrieved.
#[derive(Message, Debug)]
#[rtype(result = "Result<Vec<LabeledTypedTuple>, Error>")]
pub struct Project {
    pub(crate) input: Vec<LabeledTypedTuple>,
    pub(crate) columns: Vec<String>,
}

/// A message issued to the engine requesting that combines two tables on their matching column names.
#[derive(Message, Debug)]
#[rtype(result = "Result<Vec<LabeledTypedTuple>, Error>")]
pub struct Join {
    pub(crate) input_1: Vec<LabeledTypedTuple>,
    pub(crate) input_2: Vec<LabeledTypedTuple>,
    pub(crate) cond: Cmp,
}
