use crate::tables::*;
use crate::types::*;
use crate::*;
use proc_macro2::TokenStream;
use quote::quote;
use std::collections::*;

#[derive(Debug)]
pub struct Interface {
    pub name: TypeName,
    pub interfaces: Vec<RequiredInterface>,
}

impl Interface {
    pub fn from_type_def(reader: &TypeReader, def: TypeDef) -> Self {
        let name = TypeName::from_type_def(reader, def);
        let mut interfaces = RequiredInterface::all(reader, &name);

        let mut default_interface = RequiredInterface::from_type_def(reader, def);
        default_interface.kind = InterfaceKind::Default;
        interfaces.push(default_interface);

        Self { name, interfaces }
    }

    pub fn dependencies(&self) -> Vec<TypeDef> {
        let mut dependencies = Vec::new();

        for interface in &self.interfaces {
            dependencies.append(&mut interface.name.dependencies());

            if interface.kind == InterfaceKind::Default {
                for method in &interface.methods {
                    dependencies.append(&mut method.dependencies());
                }
            }
        }

        dependencies
    }

    pub fn to_tokens(&self) -> TokenStream {
        let name = self.name.to_tokens(&self.name.namespace);
        let abi_name = self.name.to_abi_tokens(&self.name.namespace);
        let phantoms = self.name.phantoms();
        let constraints = self.name.constraints();
        let default_interface = self.interfaces.last().unwrap();
        let guid = default_interface.guid.to_tokens();

        let projected_methods = self.projected_methods();

        let abi_methods = default_interface
            .methods
            .iter()
            .map(|method| method.to_abi_tokens(&default_interface.name.namespace));

        quote! {
            #[repr(transparent)]
            #[derive(Default, Clone)]
            pub struct #name where #constraints {
                ptr: ::winrt::IUnknown,
                #phantoms
            }
            impl<#constraints> #name {
                #projected_methods
            }
            unsafe impl<#constraints> ::winrt::ComInterface for #name {
                const GUID: ::winrt::Guid = ::winrt::Guid::from_values(#guid);
            }
            #[repr(C)]
            pub struct #abi_name where #constraints {
                __base: [usize; 6],
                #(#abi_methods)*
                #phantoms
            }

        }
    }

    // TODO: this should share an implementation with interface methods
    fn projected_methods(&self) -> TokenStream {
        let mut names = BTreeSet::new();

        // Must start with the default interface to avoid dropping methods from the default interface due to a collision.
        debug_assert!(self.interfaces.last().unwrap().kind == InterfaceKind::Default);

        for interface in self.interfaces.iter().rev() {
            for method in &interface.methods {
                // If there are any collisions just drop and caller can QI for the right interface.
                if names.contains(&method.name) {
                    continue;
                }

                names.insert(&method.name);
                //let method_name = format_ident(&method.name);
            }
        }

        quote! {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn interface((namespace, type_name): (&str, &str)) -> Interface {
        let reader = &TypeReader::from_os();
        let t = reader.resolve_type((namespace, type_name));

        match t {
            Type::Interface(t) => t,
            _ => panic!("Type not an interface"),
        }
    }

    #[test]
    fn test_stringable() {
        let t = interface(("Windows.Foundation", "IStringable"));
        assert!(t.name.runtime_name() == "Windows.Foundation.IStringable");
        assert!(t.interfaces.len() == 1);
        let t = &t.interfaces[0];
        assert!(t.name.runtime_name() == "Windows.Foundation.IStringable");
        assert!(t.kind == InterfaceKind::Default);

        assert!(t.methods.len() == 1);
        let method = &t.methods[0];
        assert!(method.name == "to_string");
        assert!(method.kind == MethodKind::Normal);

        assert!(method.params.is_empty());
        let param = method.return_type.as_ref().unwrap();
        assert!(param.kind == TypeKind::String);

        assert!(format!("{:#?}", &t.guid) == "96369F54-8EB6-48F0-ABCE-C1B211E627C3");
    }

    #[test]
    fn test_async_action() {
        let t = interface(("Windows.Foundation", "IAsyncAction"));
        assert!(t.name.runtime_name() == "Windows.Foundation.IAsyncAction");

        assert!(t.interfaces.len() == 2);

        let interface = t
            .interfaces
            .iter()
            .find(|interface| interface.name.name == "IAsyncInfo")
            .unwrap();

        assert!(interface.kind == InterfaceKind::NonDefault);
        assert!(interface.name.runtime_name() == "Windows.Foundation.IAsyncInfo");

        let interface = t
            .interfaces
            .iter()
            .find(|interface| interface.name.name == "IAsyncAction")
            .unwrap();

        assert!(interface.kind == InterfaceKind::Default);
        assert!(interface.name.runtime_name() == "Windows.Foundation.IAsyncAction");
    }

    #[test]
    fn test_observable_map() {
        let t = interface(("Windows.Foundation.Collections", "IObservableMap`2"));
        assert!(t.name.runtime_name() == "Windows.Foundation.Collections.IObservableMap`2<K, V>");
        assert!(t.interfaces.len() == 3);

        let default_interface = t
            .interfaces
            .iter()
            .find(|required| required.name.name == "IObservableMap`2")
            .unwrap();

        assert!(default_interface.kind == InterfaceKind::Default);
        assert!(default_interface.methods.len() == 2);
        assert!(default_interface.methods[0].name == "map_changed");
        assert!(default_interface.methods[1].name == "remove_map_changed");

        let map = t
            .interfaces
            .iter()
            .find(|required| required.name.name == "IMap`2")
            .unwrap();

        assert!(map.kind == InterfaceKind::NonDefault);
        assert!(map.name.runtime_name() == "Windows.Foundation.Collections.IMap`2<K, V>");

        let iterable = t
            .interfaces
            .iter()
            .find(|required| required.name.name == "IIterable`1")
            .unwrap();

        assert!(iterable.kind == InterfaceKind::NonDefault);
        assert!(iterable.name.runtime_name() == "Windows.Foundation.Collections.IIterable`1<Windows.Foundation.Collections.IKeyValuePair`2<K, V>>");
    }
}
