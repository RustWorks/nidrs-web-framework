use syn::Error;
use syn_args::{def, derive::ArgsParse, ArgsParse, Formal};

#[derive(Debug, PartialEq, ArgsParse)]
pub enum ModuleArgs {
    F1(def::Int, def::Int),
    F2(def::Int),
    F3(def::Ident),
    F4(def::Array<def::Ident>),
    F5(ModuleSubObj),
    F6(def::Array<ModuleSubObj>),
}

#[derive(Debug, PartialEq, ArgsParse)]
pub struct ModuleSubObj {
    pub imports: def::Array<def::Ident>,
    // pub interceptors: def::Array<def::Ident>,
    // pub controllers: def::Array<def::Ident>,
    // pub services: def::Array<def::Ident>,
    // pub exports: def::Array<def::Ident>,
}

fn test_formal_f1() {
    let f = Formal::new();

    // fm.parse("F(Object{ a: Int, b: Optional(Int) }, Array(Int))");
    let args = f.parse("F(1, 3)").unwrap();
    // let args = f.parse("F(1)").unwrap();
    // let args = f.parse("F(Hello)").unwrap();
    println!("{:?}", args);

    let res = ModuleArgs::try_from(&args).unwrap();
    println!("{:?}", res);

    assert_eq!(res, ModuleArgs::F1(def::Int(1), def::Int(3)));
}

fn test_formal_f2() {
    let f = Formal::new();

    let args = f.parse("F(1)").unwrap();
    // let args = f.parse("F(Hello)").unwrap();
    println!("{:?}", args);

    let res = ModuleArgs::try_from(&args).unwrap();
    println!("{:?}", res);

    assert_eq!(res, ModuleArgs::F2(def::Int(1)));
}

fn test_formal_f3() {
    let res = ModuleArgs::parse("F(Hello)").unwrap();
    println!("{:?}", res);

    assert_eq!(res, ModuleArgs::F3(def::Ident("Hello".to_string())));
}

fn test_formal_f4() {
    let res = ModuleArgs::parse("F([Ident1, Ident2])").unwrap();
    println!("{:?}", res);

    assert_eq!(res, ModuleArgs::F4(def::Array(vec![def::Ident("Ident1".to_string()), def::Ident("Ident2".to_string())])));
}
fn test_formal_f5() {
    let res = ModuleArgs::parse("F({ imports: [Ident1, Ident2] })").unwrap();
    println!("{:?}", res);

    assert_eq!(res, ModuleArgs::F5(ModuleSubObj { imports: def::Array(vec![def::Ident("Ident1".to_string()), def::Ident("Ident2".to_string())]) }));
}

fn test_formal_f6() {
    let res = ModuleArgs::parse("F([{ imports: [Ident1, Ident2] }, { imports: [Ident3, Ident4] }])").unwrap();
    println!("{:?}", res);

    assert_eq!(
        res,
        ModuleArgs::F6(def::Array(vec![
            ModuleSubObj { imports: def::Array(vec![def::Ident("Ident1".to_string()), def::Ident("Ident2".to_string())]) },
            ModuleSubObj { imports: def::Array(vec![def::Ident("Ident3".to_string()), def::Ident("Ident4".to_string())]) }
        ]))
    );
}
fn main() {
    test_formal_f1();
    test_formal_f2();
    test_formal_f3();
    test_formal_f4();
    test_formal_f5();
    test_formal_f6();
}