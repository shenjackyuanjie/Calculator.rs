use self::modules::{basic, bit_ops, file_system, math};
use super::value::{
    oop::{class::Class, object::Object},
    value::Value,
};
use crate::public::std::modules::{array, string};

pub mod modules;

mod utils;

#[derive(PartialEq, Clone, Copy)]
pub enum StdModules {
    Basic,
    Math,
    Array,
    String,
    FileSystem,
    BitOps,
}

impl StdModules {
    pub fn get_fn_list(&self) -> Vec<(String, Value)> {
        match self {
            StdModules::Basic => basic::function_list(),
            StdModules::BitOps => bit_ops::function_list(),
            _ => unreachable!(),
        }
    }

    pub fn get_obj_entry(&self) -> Object {
        match self {
            StdModules::Math => math::module_object(),
            StdModules::FileSystem => file_system::module_object(),
            _ => unreachable!(),
        }
    }

    pub fn get_cls_entry(&self) -> Class {
        match self {
            StdModules::Array => array::module_class(),
            StdModules::String => string::module_class(),
            _ => unreachable!(),
        }
    }
}
