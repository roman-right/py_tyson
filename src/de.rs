use pest::iterators::Pair;
use pest::Parser;
use crate::document::Document;

use crate::errors::TySONError;
use crate::item::{Item, ItemStruct};
use crate::map::MapItem;
use crate::modifier::ModifierItem;
use crate::primitive::PrimitiveItem;
use crate::vector::VectorItem;

#[derive(Parser)]
#[grammar = "tyson.pest"]
struct TySONParser;


pub trait Desereilize {
    fn deserialize_primitive(&self, pair: Pair<Rule>) -> PrimitiveItem {
        let mut data: String = String::new();
        let mut prefix: String = String::new();
        for pair in pair.into_inner() {
            match pair.as_rule() {
                Rule::prefix => {
                    prefix = pair.as_str().to_string();
                }
                _ => {
                    data = pair.as_str().to_string();
                }
            }
        };
        Self::new_primitive(&self, prefix, data)
    }

    fn deserialize_modifier(&self, pair: Pair<Rule>) -> Result<ModifierItem, TySONError> {
        let mut inner_rules = pair.into_inner();
        let prefix = inner_rules.next().ok_or(TySONError::unexpected_parsing())?.as_str().to_string();
        let pair = inner_rules.next().ok_or(TySONError::unexpected_parsing())?;
        Ok(Self::new_modifier(prefix, Self::route_deserialization(&self, pair)?))
    }

    fn deserialize_vector(&self, pair: Pair<Rule>) -> Result<VectorItem, TySONError> {
        let mut inner_rules = pair.into_inner();
        let prefix = inner_rules.next().ok_or(TySONError::unexpected_parsing())?.as_str().to_string();
        let mut vector = Self::new_vector(prefix);
        for pair in inner_rules
        {
            vector.push(Self::route_deserialization(&self, pair)?);
        }
        Ok(vector)
    }

    fn deserialize_map(&self, pair: Pair<Rule>) -> Result<MapItem, TySONError> {
        let mut inner_rules = pair.into_inner();
        let prefix = inner_rules.next().ok_or(TySONError::unexpected_parsing())?.as_str().to_string();
        let mut map = Self::new_map(prefix);
        for pair in inner_rules
        {
            let mut inner_rules = pair.into_inner();
            let left = Self::deserialize_primitive(&self, inner_rules.next().ok_or(TySONError::unexpected_parsing())?);
            map.insert(left, Self::route_deserialization(&self, inner_rules.next().ok_or(TySONError::unexpected_parsing())?)?);
        };
        Ok(map)
    }

    fn route_deserialization(&self, pair: Pair<Rule>) -> Result<ItemStruct, TySONError> {
        return match pair.as_rule() {
            Rule::map => {
                let res = Self::deserialize_map(&self, pair)?;
                Ok(ItemStruct::from(Item::Map(res)))
            }
            Rule::vector => {
                let res = Self::deserialize_vector(&self, pair)?;
                Ok(ItemStruct::from(Item::Vector(res)))
            }
            Rule::modifier => {
                let res = Self::deserialize_modifier(&self, pair)?;
                Ok(ItemStruct::from(Item::Modifier(res)))
            }
            Rule::primitive => {
                let res = Self::deserialize_primitive(&self, pair);
                Ok(ItemStruct::from(Item::Primitive(res)))
            }
            _ => { Err(TySONError::new("Deserialization error")) }
        };
    }


    fn deserialize(data: String) -> Result<Document, TySONError> {
        let pair = TySONParser::parse(Rule::document, data.as_str())?.next().ok_or(TySONError::unexpected_parsing())?;

        let mut result = Self::new_document();

        match pair.as_rule() {
            Rule::document => {
                for pair in pair.into_inner() {
                    let mut inner_rules = pair.into_inner();
                    match inner_rules.next() {
                        Some(v) => {
                            let key = result.deserialize_primitive(v);
                            result.add_to_document((key, result.route_deserialization(inner_rules.next().ok_or(TySONError::unexpected_parsing())?)?));
                        }
                        _ => {}
                    }
                }
                Ok(result)
            }
            _ => { Err(TySONError::new("Deserialization error")) }
        }
    }

    fn new_document() -> Document;

    fn add_to_document(&mut self, data: (PrimitiveItem, ItemStruct));

    fn new_modifier(prefix: String, data: ItemStruct) -> ModifierItem {
        ModifierItem::new(prefix, data)
    }

    fn new_vector(prefix: String) -> VectorItem {
        VectorItem::new(prefix.to_string())
    }

    fn new_map(prefix: String) -> MapItem {
        MapItem::new(prefix.to_string())
    }

    fn new_primitive(&self, prefix: String, data: String) -> PrimitiveItem {
        PrimitiveItem::new(prefix, data)
    }
}
