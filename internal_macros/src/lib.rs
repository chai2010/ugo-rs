#![allow(unused)]

use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

#[proc_macro_attribute]
pub fn runtime_fn(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let parsed_fn = parse_macro_input!(item as syn::ItemFn);

    if let Ok(_) = std::env::var("UGO_GEN_BUILTIN_API") {
        print_api_spec(&parsed_fn);
    }

    let result = quote! {
        #parsed_fn
    };
    result.into()
}

fn print_api_spec(fn_item: &syn::ItemFn) {
    let _fn_name = get_fn_name(fn_item);
    let _fn_sig = get_fn_sig(fn_item);
}

fn get_fn_name(fn_item: &syn::ItemFn) -> String {
    fn_item.sig.ident.to_string()
}

fn get_fn_sig(fn_item: &syn::ItemFn) -> String {
    let fn_name = get_fn_name(fn_item);
    let args_type = get_fn_args_type(fn_item);
    let output_type = get_fn_output_type(fn_item);

    if output_type != "" {
        format!("// func {} ({}) {}", fn_name, args_type, output_type)
    } else {
        format!("// func {} ({})", fn_name, args_type)
    }
}

fn get_fn_args_type(fn_item: &syn::ItemFn) -> String {
    let fn_name = get_fn_name(fn_item);
    let inputs = &fn_item.sig.inputs;

    let mut result = String::new();
    for (i, arg) in inputs.iter().enumerate() {
        let arg_name = get_fn_arg_name(arg);
        let arg_type = get_fn_arg_type(arg);

        if i > 0 {
            result.push_str(", ");
        }
        result.push_str(format!("{} {}", arg_name, arg_type).as_ref());
    }
    result
}

fn get_fn_output_type(fn_item: &syn::ItemFn) -> String {
    match &fn_item.sig.output {
        syn::ReturnType::Type(_, ty) => build_type(&ty),
        _ => "".to_string(),
    }
}

fn get_fn_arg_name(arg: &syn::FnArg) -> String {
    match arg {
        syn::FnArg::Typed(ty) => match ty {
            syn::PatType { pat, .. } => match &**pat {
                syn::Pat::Ident(x) => x.ident.to_string(),
                _ => "".to_string(),
            },
            _ => "".to_string(),
        },
        _ => "".to_string(),
    }
}

fn get_fn_arg_type(arg: &syn::FnArg) -> String {
    match arg {
        syn::FnArg::Typed(ty) => match ty {
            syn::PatType { ty, .. } => build_type(ty),
            _ => "".to_string(),
        },
        _ => "".to_string(),
    }
}

fn build_type(ty: &syn::Type) -> String {
    match ty {
        syn::Type::Path(ty_path) => {
            let ty_name = ty_path.path.segments[0].ident.to_string();
            match ty_name.as_ref() {
                "c_void" => "".to_string(),
                "c_char" => "char".to_string(),
                "i8" | "u8" => "int8".to_string(),
                "i16" | "u16" => "int16".to_string(),
                "i32" | "u32" => "int32".to_string(),
                "i64" | "u64" => "int64".to_string(),
                "f32" => "float32".to_string(),
                "f64" => "float64".to_string(),
                _ => ty_name,
            }
        }
        syn::Type::Ptr(ty_ptr) => {
            let s = build_type(&ty_ptr.elem);
            format!("*{}", s)
        }
        syn::Type::Reference(ty_ref) => {
            let s = build_type(&ty_ref.elem);
            format!("*{}", s)
        }
        syn::Type::BareFn(_) => {
            "todo".to_string()
        }
        syn::Type::Never(_) => {
            "todo".to_string()
        }
        _ => panic!("unsupport type: {}", quote!(#ty)),
    }
}
