use std::{
    collections::{BTreeMap, BTreeSet},
    path::PathBuf,
};

use case::CaseExt;
use clap::Parser;
use jassparse::{is_agent_type, is_enum_type, JassParser, Rule};

#[derive(Default, Debug, Clone, PartialEq, Eq, Hash)]
struct JassHandleType {
    name:         String,
    derived_from: String,
}

#[derive(Default, Debug, Clone)]
struct StringJassNative {
    name:      String,
    params:    Vec<StringJassParam>,
    return_ty: String,
}

#[derive(Default, Debug, Clone)]
struct StringJassParam {
    name: String,
    ty:   String,
}

#[derive(Default, Debug, Clone)]
struct Param<T> {
    name: String,
    ty:   T,
}

#[derive(Default, Debug, Clone)]
struct Native<T> {
    name:      String,
    params:    Vec<Param<T>>,
    return_ty: T,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
enum JassType {
    Int,
    Real,
    Bool,
    String,
    Code,
    Handle(String),
    #[default]
    Nothing,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
enum RustType {
    I32,
    F32,
    ConstI8Ptr,
    MutI8Ptr,
    MutUsizePtr,
    Bool,
    CodeId,
    Handle(String),
    AgentRef(String),
    Weak(String),
    CStr,
    // CString,
    String,
    #[default]
    Void,
}

impl RustType {
    fn to_source(&self) -> String {
        match self {
            RustType::I32 => "i32".to_string(),
            RustType::F32 => "f32".to_string(),
            RustType::ConstI8Ptr => "*const i8".to_string(),
            RustType::MutI8Ptr => "*mut i8".to_string(),
            RustType::MutUsizePtr => "*mut usize".to_string(),
            RustType::Bool => "bool".to_string(),
            RustType::CodeId => "CodeId".to_string(),
            RustType::Handle(handle) => format!("Handle<j_{}>", handle),
            RustType::AgentRef(agent) => format!("&Agent<j_{}>", agent),
            RustType::Weak(agent) => format!("Option<Weak<j_{}>>", agent),
            RustType::CStr => "&CStr".to_string(),
            // RustType::CString => "CString".to_string(),
            RustType::String => "String".to_string(),
            RustType::Void => "()".to_string(),
        }
    }
}

fn parse_native(decl: &StringJassNative) -> Native<JassType> {
    let mut native = Native::default();

    for param in &decl.params {
        let ty = match param.ty.as_str() {
            "integer" => JassType::Int,
            "real" => JassType::Real,
            "boolean" => JassType::Bool,
            "string" => JassType::String,
            "code" => JassType::Code,
            "handle" => JassType::Handle("handle".to_string()),
            other => JassType::Handle(other.to_string()),
        };

        native.params.push(Param {
            name: param.name.clone(),
            ty,
        });
    }

    native.return_ty = match decl.return_ty.as_str() {
        "nothing" => JassType::Nothing,
        "integer" => JassType::Int,
        "real" => JassType::Real,
        "boolean" => JassType::Bool,
        "string" => JassType::String,
        "code" => JassType::Code,
        "handle" => JassType::Handle("handle".to_string()),
        other => JassType::Handle(other.to_string()),
    };

    native.name = decl.name.clone();

    native
}

fn should_escape_ident(ident: &str) -> bool {
    matches!(ident, "where")
}

fn remap_native_to_rust(jass_native: &Native<JassType>, is_raw: bool) -> Native<RustType> {
    let mut native = Native {
        name: jass_native.name.clone(),
        ..Default::default()
    };

    for param in &jass_native.params {
        let ty = match &param.ty {
            JassType::Int => RustType::I32,
            JassType::Real => RustType::F32,
            JassType::Bool => RustType::Bool,
            JassType::String => {
                if is_raw {
                    RustType::ConstI8Ptr
                } else {
                    RustType::CStr
                }
            }
            JassType::Code => RustType::CodeId,
            JassType::Handle(handle) => {
                if is_enum_type(handle) {
                    RustType::I32
                } else if is_raw {
                    RustType::Handle(handle.clone())
                } else if is_agent_type(handle) {
                    RustType::AgentRef(handle.clone())
                } else {
                    RustType::Handle(handle.clone())
                }
            }
            JassType::Nothing => {
                panic!("'nothing' is not a valid parameter type")
            }
        };

        let param_name = if should_escape_ident(&param.name) {
            format!("_{}", param.name)
        } else {
            param.name.clone()
        };

        native.params.push(Param {
            name: param_name.to_snake(),
            ty,
        });
    }

    if is_raw && jass_native.return_ty == JassType::String {
        native.params.push(Param {
            name: "out".to_string(),
            ty:   RustType::MutI8Ptr,
        });

        native.params.push(Param {
            name: "out_len".to_string(),
            ty:   RustType::MutUsizePtr,
        });

        native.return_ty = RustType::Void;
    } else if !is_raw && jass_native.return_ty == JassType::String {
        native.return_ty = RustType::String;
    } else if !is_raw && matches!(jass_native.return_ty, JassType::Handle(_)) {
        let name = match &jass_native.return_ty {
            JassType::Handle(handle) => handle,
            _ => unreachable!(),
        };

        if is_agent_type(name) {
            native.return_ty = RustType::Weak(name.clone());
        } else if is_enum_type(name) {
            native.return_ty = RustType::I32;
        } else {
            native.return_ty = RustType::Handle(name.clone());
        }
    } else {
        native.return_ty = match &jass_native.return_ty {
            JassType::Int => RustType::I32,
            JassType::Real => RustType::F32,
            JassType::Bool => RustType::Bool,
            JassType::Code => RustType::CodeId,
            JassType::Handle(handle) => {
                if is_enum_type(handle) {
                    RustType::I32
                } else {
                    RustType::Handle(handle.clone())
                }
            }
            JassType::Nothing => RustType::Void,
            other => panic!("{other:?} is not a valid return type"),
        };
    }

    native
}

fn emit_native(j_native: &Native<JassType>) {
    let rs_native_raw = remap_native_to_rust(j_native, true);
    let rs_native_user = remap_native_to_rust(j_native, false);

    println!("        {{");
    println!("            link: \"{}\"", rs_native_raw.name);
    println!("            name: {}", rs_native_raw.name.to_snake());
    print!("            raw: (");

    for (i, param) in rs_native_raw.params.iter().enumerate() {
        if i != 0 {
            print!(", ");
        }

        print!("{}: {}", param.name, param.ty.to_source());
    }

    print!(")");

    if rs_native_raw.return_ty != RustType::Void {
        print!(" -> {}", rs_native_raw.return_ty.to_source());
    }

    println!(";");

    print!("            user: (");

    for (i, param) in rs_native_user.params.iter().enumerate() {
        if i != 0 {
            print!(", ");
        }

        print!("{}: {}", param.name, param.ty.to_source());
    }

    print!(")");

    if rs_native_user.return_ty != RustType::Void {
        print!(" -> {}", rs_native_user.return_ty.to_source());
    }

    println!(" {{");
    print!("                ");

    print!("raw::{}(", rs_native_raw.name.to_snake());
    for (i, param) in j_native.params.iter().enumerate() {
        let param_name = if should_escape_ident(&param.name) {
            format!("_{}", param.name)
        } else {
            param.name.clone()
        };

        if i != 0 {
            print!(", ");
        }

        if param.ty == JassType::String {
            print!("{}.as_ptr()", param_name.to_snake());
        } else if let JassType::Handle(h) = &param.ty {
            if is_agent_type(h) {
                print!("{}.to_handle()", param_name.to_snake());
            } else {
                print!("{}", param_name.to_snake());
            }
        } else {
            print!("{}", param_name.to_snake());
        }
    }

    if j_native.return_ty == JassType::String {
        if !j_native.params.is_empty() {
            print!(", ");
        }

        print!("stringret::ptr(), stringret::reset_len()");
    }

    print!(")");

    if j_native.return_ty == JassType::String {
        print!("; stringret::read_string()");
    }

    if let JassType::Handle(h) = &j_native.return_ty {
        if is_agent_type(h) {
            print!(".to_weak()");
        }
    }

    println!();
    println!("            }};");
    println!("        }},");
}

#[derive(clap::Parser)]
struct Cli {
    input:   PathBuf,
    #[clap(subcommand)]
    command: Command,
}

#[derive(clap::Subcommand)]
#[allow(clippy::enum_variant_names)]
enum Command {
    GenerateRsSys,
    GenerateRsConsts,
    GenerateCs,
}

fn emit_ffi_macro(natives: &[StringJassNative]) {
    println!("#[rustfmt::skip]");
    println!("pub mod natives {{");
    println!("    use super::*;");
    println!("    crate::declare_ffi! {{");
    for native in natives {
        let jass_native = parse_native(native);
        emit_native(&jass_native);
    }
    println!("    }}");
    println!("}}");
}

fn emit_handles(handle_types: &[JassHandleType], allsubtypes: &BTreeMap<String, BTreeSet<String>>) {
    println!("#[rustfmt::skip]");
    println!("pub(crate) mod handles {{");
    println!("    use super::*;");

    println!("    crate::define_handles! {{");
    for handle_type in handle_types {
        let parents = allsubtypes.get(&handle_type.name).unwrap();
        println!(
            "        j_{}: ({}),",
            handle_type.name,
            parents
                .iter()
                .map(|parent| format!("j_{}", parent))
                .collect::<Vec<_>>()
                .join(", ")
        );
    }

    println!("    }}");
    println!("}}");
}

fn main() {
    let cli = Cli::parse();

    let input = std::fs::read_to_string(cli.input).expect("failed to read input file");

    let pairs =
        <JassParser as pest::Parser<_>>::parse(Rule::jass, &input).expect("failed to parse input");

    let mut natives = Vec::new();
    let mut handle_types = Vec::new();
    let mut subtypes = BTreeMap::new();
    let mut allsubtypes = BTreeMap::new();
    let mut globals = Vec::new();

    for pair in pairs.flatten() {
        if pair.as_rule() == Rule::nativedecl {
            let mut native = StringJassNative::default();
            let inner: Vec<_> = pair.into_inner().collect();

            let name = &inner[0];
            let param_list = &inner[1];
            let return_ty = &inner[2];

            native.name = name.as_str().to_string();

            {
                let inner: Vec<_> = param_list.clone().into_inner().collect();
                for pair in inner {
                    let mut param = StringJassParam::default();
                    let inner: Vec<_> = pair.into_inner().collect();
                    param.name = inner[1].as_str().to_string();
                    param.ty = inner[0].as_str().to_string();
                    native.params.push(param);
                }
            }

            native.return_ty = return_ty.as_str().to_string();
            natives.push(native);
        } else if pair.as_rule() == Rule::typedecl {
            let mut handle_type = JassHandleType::default();
            let inner: Vec<_> = pair.into_inner().collect();

            let name = &inner[0];
            let derived_from = &inner[1];

            handle_type.name = name.as_str().to_string();
            handle_type.derived_from = derived_from.as_str().to_string();

            subtypes.insert(handle_type.name.clone(), handle_type.derived_from.clone());
            allsubtypes.insert(
                handle_type.name.clone(),
                vec![handle_type.derived_from.clone()]
                    .into_iter()
                    .collect::<BTreeSet<_>>(),
            );
            let mut parent = handle_type.derived_from.clone();
            while parent != "handle" {
                let derived_from = subtypes.get(&parent).unwrap().clone();
                allsubtypes
                    .get_mut(&handle_type.name)
                    .unwrap()
                    .insert(derived_from.clone());
                parent = derived_from.clone();
            }

            handle_types.push(handle_type);
        } else if pair.as_rule() == Rule::globaldecl {
            let mut inner: Vec<_> = pair.into_inner().collect();
            let name = inner[1].as_str();
            let call = inner.remove(2).into_inner().collect::<Vec<_>>();

            if call.len() == 2 {
                let value = call[1].as_str();
                globals.push((name.to_string(), value.to_string()))
            }
        }
    }

    match cli.command {
        Command::GenerateRsSys => {
            println!("#![allow(clippy::should_implement_trait)]");
            println!("#![allow(clippy::style)]");
            println!("#![allow(clippy::too_many_arguments)]");
            println!("#![allow(non_camel_case_types)]");

            println!("use handles::*;");
            println!("use crate::base::Handle;");
            println!("use crate::base::j_handle;");
            println!("use crate::base::CodeId;");

            emit_ffi_macro(&natives);
            emit_handles(&handle_types, &allsubtypes);

            println!();
        }
        Command::GenerateRsConsts => {
            for global in &globals {
                println!("pub const {}: i32 = {};", global.0, global.1);
            }
        }
        Command::GenerateCs => {
            println!("// agent subtypes ");
            println!("switch (ty) {{");

            for handle_type in &handle_types {
                let parents = allsubtypes.get(&handle_type.name).unwrap();

                if parents.contains("agent") {
                    println!("case j_{}:", handle_type.name);
                }
            }
            //
        }
    }
}
