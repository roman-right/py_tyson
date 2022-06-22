use std::collections::HashMap;
use crate::map::MapItem;
use crate::modifier::ModifierItem;
use crate::primitive::PrimitiveItem;
use crate::vector::VectorItem;

use pyo3::prelude::*;

#[derive(Clone, Debug)]
pub enum Item {
    Primitive(PrimitiveItem),
    Modifier(ModifierItem),
    Vector(VectorItem),
    Map(MapItem),
}


#[derive(Debug, Clone)]
#[pyclass]
pub struct ItemStruct {
    pub(crate) value: Item,
}


#[pymethods]
impl ItemStruct {
    pub fn primitive(&self) -> Option<PrimitiveItem> {
        match &self.value {
            Item::Primitive(o) => { Some(o.clone()) }
            _ => { None }
        }
    }

    pub fn modifier(&self) -> Option<ModifierItem> {
        match &self.value {
            Item::Modifier(o) => { Some(o.clone()) }
            _ => { None }
        }
    }

    pub fn vector(&self) -> Option<VectorItem> {
        match &self.value {
            Item::Vector(o) => { Some(o.clone()) }
            _ => { None }
        }
    }

    pub fn map(&self) -> Option<MapItem> {
        match &self.value {
            Item::Map(o) => { Some(o.clone()) }
            _ => { None }
        }
    }
}

impl From<Item> for ItemStruct {
    fn from(o: Item) -> Self {
        ItemStruct {
            value: o
        }
    }
}