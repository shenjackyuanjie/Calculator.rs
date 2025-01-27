use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;

use crate::public::error::{reference_error, ReferenceType};
use crate::public::value::array::{Array, ArrayLiteral};
use crate::public::value::oop::class::Class;

use super::super::display_indent;
use super::super::value::Value;
use super::utils::data_storage::DataStoragePattern;
use super::utils::getter::getter;

#[derive(PartialEq, Clone)]
pub struct Object {
    pub prototype: Rc<Class>,

    pub storage_pattern: DataStoragePattern,
    pub data_list: Option<Vec<(String, Rc<RefCell<Value>>)>>,
    pub data_map: Option<HashMap<String, Rc<RefCell<Value>>>>,
}

impl Object {
    pub fn get(&self, prop_name: &str) -> Result<Value, ()> {
        let target_value_result = getter::<Rc<RefCell<Value>>>(
            self.storage_pattern,
            prop_name,
            &self.data_list,
            &self.data_map,
        );
        match target_value_result {
            Ok(target_rc) => {
                let target_ref = target_rc.as_ref().borrow();
                Ok(target_ref.unwrap())
            }
            Err(_) => {
                let target_method = self.prototype.get_method(prop_name)?;
                Ok(Value::Function(target_method.clone()))
            }
        }
    }

    pub fn set(&self, prop_name: &String, value: Value) -> Result<(), ()> {
        let result_target_rc = getter::<Rc<RefCell<Value>>>(
            self.storage_pattern,
            prop_name,
            &self.data_list,
            &self.data_map,
        );

        match result_target_rc {
            Ok(target_rc) => {
                let mut target_ref = target_rc.as_ref().borrow_mut();
                *target_ref = value;
                Ok(())
            }
            Err(()) => Err(reference_error(ReferenceType::Property, prop_name)?),
        }
    }

    pub fn display(
        f: &mut fmt::Formatter<'_>,
        obj: &Rc<RefCell<Object>>,
        level: usize,
    ) -> fmt::Result {
        fn display_item(
            f: &mut fmt::Formatter<'_>,
            key: &str,
            value: &Rc<RefCell<Value>>,
            level: usize,
        ) -> fmt::Result {
            let value_ref = value.as_ref().borrow();

            // print indent and key
            write!(f, "{}{}: ", display_indent(level), key)?;

            // print value
            match &*value_ref {
                Value::String(_) => write!(f, "{}", value_ref.str_format())?,
                Value::Array(arr) => Array::display(f, arr, level + 1)?,
                Value::Object(obj) => Object::display(f, obj, level + 1)?,
                _ => write!(f, "{}", value_ref)?,
            }

            // next line
            write!(f, "\r\n")
        }

        let obj_ref = obj.as_ref().borrow();

        write!(f, "{{\r\n")?;
        match obj_ref.storage_pattern {
            DataStoragePattern::List => {
                let list = obj_ref.data_list.as_ref().unwrap();
                for (k, v) in list {
                    display_item(f, k, v, level)?;
                }
            }
            DataStoragePattern::Map => {
                let map = obj_ref.data_map.as_ref().unwrap();

                for (k, v) in map {
                    display_item(f, k, v, level)?;
                }
            }
        }
        Class::display_methods(f, &obj_ref.prototype, level)?;
        write!(f, "{}}}", display_indent(level - 1))
    }

    pub fn deep_clone(obj: Rc<RefCell<Object>>) -> Value {
        fn prop_value_resolve(v: &Rc<RefCell<Value>>, param_vec: &mut ArrayLiteral) {
            let v_ref = v.as_ref().borrow();
            if let Value::Object(sub_obj) = v_ref.unwrap() {
                param_vec.push_back(Object::deep_clone(sub_obj));
            } else {
                param_vec.push_back(v_ref.deep_clone());
            }
        }

        let obj_ref = &*(obj.as_ref().borrow());
        let mut instantiation_params = ArrayLiteral::new();

        match obj_ref.storage_pattern {
            DataStoragePattern::List => {
                if let Some(list) = &obj_ref.data_list {
                    for (_, v) in list {
                        prop_value_resolve(v, &mut instantiation_params);
                    }
                }
            }
            DataStoragePattern::Map => {
                if let Some(map) = &obj_ref.data_map {
                    for (_, v) in map {
                        prop_value_resolve(v, &mut instantiation_params);
                    }
                }
            }
        }

        // the object has been passed the type check before,
        // thus with properties of the object,
        // the instantiation must pass the type check.
        let res_object =
            Class::instantiate(obj_ref.prototype.clone(), instantiation_params).unwrap();
        return Value::from(res_object);
    }
}
