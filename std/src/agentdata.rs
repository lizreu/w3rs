use core::hash::Hash;
use std::cell::{Ref, RefCell, RefMut};

use rustc_hash::FxHashMap;
use w3_sys::{j_agent, Agent, JassHandle, SubtypeOf};

pub trait AssociatedData {
    type AgentType: SubtypeOf<j_agent>;
    type ValueType: Sized + 'static;

    #[doc(hidden)]
    unsafe fn get_map(
    ) -> &'static RefCell<FxHashMap<w3_sys::Agent<Self::AgentType>, Self::ValueType>>;
}

pub trait AgentEx<T>
where
    T: JassHandle + SubtypeOf<j_agent> + 'static,
{
    fn get_data_mut<A: AssociatedData<AgentType = T>>(&self) -> Option<RefMut<A::ValueType>>;
    fn get_data<A: AssociatedData<AgentType = T>>(&self) -> Option<Ref<A::ValueType>>;
    fn remove_data<A: AssociatedData<AgentType = T>>(&self) -> Option<A::ValueType>;
    fn set_data<A: AssociatedData<AgentType = T>>(&self, value: A::ValueType);
}

impl<T> AgentEx<T> for Agent<T>
where
    T: JassHandle + SubtypeOf<j_agent> + 'static,
    Self: PartialEq + Eq + Hash,
{
    fn get_data_mut<A: AssociatedData<AgentType = T>>(&self) -> Option<RefMut<A::ValueType>> {
        unsafe {
            let map = A::get_map();
            let map = map.borrow_mut();

            RefMut::filter_map(map, |m| m.get_mut(self)).ok()
        }
    }

    fn get_data<A: AssociatedData<AgentType = T>>(&self) -> Option<Ref<A::ValueType>> {
        unsafe {
            let map = A::get_map();
            let map = map.borrow();

            Ref::filter_map(map, |m| m.get(self)).ok()
        }
    }

    fn remove_data<A: AssociatedData<AgentType = T>>(&self) -> Option<A::ValueType> {
        unsafe {
            let map = A::get_map();
            let mut map = map.borrow_mut();

            map.remove(self)
        }
    }

    fn set_data<A: AssociatedData<AgentType = T>>(&self, value: A::ValueType) {
        unsafe {
            let map = A::get_map();
            let mut map = map.borrow_mut();

            map.insert(self.clone(), value);
        }
    }
}

#[macro_export]
macro_rules! associated_agent_data {
    ($v:vis $marker:ident: $agenttype:ident -> $datatype:ty) => {
        $v struct $marker;

        #[doc(hidden)]
        mod __impl {
            use super::*;

            static mut AGENT_DATA: rustc_hash::FxHashMap<w3_sys::Agent<$agenttype>, $datatype> = FxHashMap::default();

            impl AssociatedData for $marker {
                type AgentType = $agenttype;
                type DataType = $datatype;

                unsafe fn get_map() -> &'static mut FxHashMap<w3_sys::Agent<Self::AgentType>, Self::DataType> {
                    if !cfg!(target_arch = "wasm32") {
                        todo!("get_map is only safe on wasm32")
                    }

                    &mut AGENT_DATA
                }
            }
        }
    };
}
