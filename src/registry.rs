use std::{any::TypeId, collections::HashMap};

use crate::modules::Module;

#[allow(clippy::type_complexity)]
#[derive(Default, Debug)]
pub struct Registry {
    modules: HashMap<TypeId, Box<dyn Module>>,
    //listeners: HashMap<TypeId, Box<dyn Listener>>,
    module_names: HashMap<String, TypeId>,
    //resolvers: HashMap<String, fn(Option<&Config>) -> Option<TypeId>>,
}
