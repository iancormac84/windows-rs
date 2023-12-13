use super::*;

pub fn from_reader(reader: &metadata::Reader, config: std::collections::BTreeMap<&str, &str>, output: &str) -> Result<()> {
    let mut writer = Writer::new(output);

    // TODO: do we need any configuration values for winmd generation?
    // Maybe per-namespace winmd files for namespace-splitting - be sure to use
    // the same key as for winmd generation.

    if let Some((key, _)) = config.first_key_value() {
        return Err(Error::new(&format!("invalid configuration value `{key}`")));
    }

    // TODO: just use the reader directly since we now have everything in the reader, there's no need to abstract
    // away the source format. Few reprs is always better.

    for item in reader.items() {
        // TODO: cover all variants
        let metadata::Item::Type(def) = item else {
            continue;
        };

        let generics = &metadata::type_def_generics(def);

        let extends = if let Some(extends) = def.extends() { writer.insert_type_ref(extends.namespace, extends.name) } else { 0 };

        writer.tables.TypeDef.push(TypeDef {
            Extends: extends,
            FieldList: writer.tables.Field.len() as u32,
            Flags: def.flags().0,
            MethodList: writer.tables.MethodDef.len() as u32,
            TypeName: writer.strings.insert(def.name()),
            TypeNamespace: writer.strings.insert(def.namespace()),
        });

        for generic in def.generics() {
            writer.tables.GenericParam.push(GenericParam {
                Number: generic.number(), // TODO: isn't this just going to be incremental?
                Flags: 0,
                Owner: TypeOrMethodDef::TypeDef(writer.tables.TypeDef.len() as u32 - 1).encode(),
                Name: writer.strings.insert(generic.name()),
            });
        }

        for interface in metadata::type_def_interfaces(def, generics) {
            let ty = winmd_type(&interface.ty);

            let reference = match &ty {
                Type::TypeRef(type_name) if type_name.generics.is_empty() => writer.insert_type_ref(&type_name.namespace, &type_name.name),
                Type::TypeRef(_) => writer.insert_type_spec(ty),
                Type::IUnknown => writer.insert_type_ref("Windows.Win32.System.Com", "IUnknown"),
                Type::IInspectable => writer.insert_type_ref("Windows.Win32.System.WinRT", "IInspectable"),
                rest => unimplemented!("{rest:?}"),
            };

            writer.tables.InterfaceImpl.push(InterfaceImpl { Class: writer.tables.TypeDef.len() as u32 - 1, Interface: reference });
        }

        // TODO: if the class is "Apis" then should we sort the fields (constants) and methods (functions) for stability

        for field in def.fields() {
            let ty = winmd_type(&field.ty(Some(def)));
            let signature = writer.insert_field_sig(&ty);

            writer.tables.Field.push(Field { Flags: field.flags().0, Name: writer.strings.insert(field.name()), Signature: signature });
        }

        for method in def.methods() {
            let signature = method.signature(generics);
            let return_type = winmd_type(&signature.return_type);
            let param_types: Vec<Type> = signature.params.iter().map(winmd_type).collect();

            let signature = writer.insert_method_sig(signature.call_flags, &return_type, &param_types);

            writer.tables.MethodDef.push(MethodDef {
                RVA: 0,
                ImplFlags: method.impl_flags().0,
                Flags: method.flags().0,
                Name: writer.strings.insert(method.name()),
                Signature: signature,
                ParamList: writer.tables.Param.len() as u32,
            });

            for param in method.params() {
                writer.tables.Param.push(Param { Flags: param.flags().0, Sequence: param.sequence(), Name: writer.strings.insert(param.name()) });
            }
        }
    }

    // TODO: In theory, `config` could instruct this function to balance the types across a number of winmd files
    // like mdmerge supports for namespace-splitting.
    write_to_file(output, writer.into_stream()).map_err(|err| err.with_path(output))
}

// TODO: keep the basic type conversion
fn winmd_type(ty: &metadata::Type) -> Type {
    match ty {
        metadata::Type::Void => Type::Void,
        metadata::Type::Bool => Type::Bool,
        metadata::Type::Char => Type::Char,
        metadata::Type::I8 => Type::I8,
        metadata::Type::U8 => Type::U8,
        metadata::Type::I16 => Type::I16,
        metadata::Type::U16 => Type::U16,
        metadata::Type::I32 => Type::I32,
        metadata::Type::U32 => Type::U32,
        metadata::Type::I64 => Type::I64,
        metadata::Type::U64 => Type::U64,
        metadata::Type::F32 => Type::F32,
        metadata::Type::F64 => Type::F64,
        metadata::Type::ISize => Type::ISize,
        metadata::Type::USize => Type::USize,
        metadata::Type::String => Type::String,
        metadata::Type::GUID => Type::GUID,
        metadata::Type::IUnknown => Type::IUnknown,
        metadata::Type::IInspectable => Type::IInspectable,
        metadata::Type::HRESULT => Type::HRESULT,
        metadata::Type::PSTR => Type::PSTR,
        metadata::Type::PWSTR => Type::PWSTR,
        metadata::Type::PCSTR => Type::PCSTR,
        metadata::Type::PCWSTR => Type::PCWSTR,
        metadata::Type::BSTR => Type::BSTR,
        metadata::Type::Type => Type::Type,
        metadata::Type::TypeDef(def, generics) => Type::TypeRef(TypeName { namespace: def.namespace().to_string(), name: def.name().to_string(), generics: generics.iter().map(winmd_type).collect() }),
        metadata::Type::GenericParam(generic) => Type::GenericParam(generic.number()),
        metadata::Type::ConstRef(ty) => Type::ConstRef(Box::new(winmd_type(ty))),
        metadata::Type::WinrtArrayRef(ty) => Type::WinrtArrayRef(Box::new(winmd_type(ty))),
        metadata::Type::WinrtArray(ty) => Type::WinrtArray(Box::new(winmd_type(ty))),
        metadata::Type::MutPtr(ty, pointers) => Type::MutPtr(Box::new(winmd_type(ty)), *pointers),
        metadata::Type::ConstPtr(ty, pointers) => Type::ConstPtr(Box::new(winmd_type(ty)), *pointers),
        metadata::Type::Win32Array(ty, len) => Type::Win32Array(Box::new(winmd_type(ty)), *len),
        rest => unimplemented!("{rest:?}"),
    }
}
