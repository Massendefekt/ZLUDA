use super::ptx;

fn parse_and_assert(s: &str) {
    let mut errors = Vec::new();
    ptx::ModuleParser::new().parse(&mut errors, s).unwrap();
    assert!(errors.len() == 0);
}

#[test]
fn empty() {
    parse_and_assert(".version 6.5 .target sm_30, debug");
}

#[test]
fn operands_ptx() {
    let vector_add = include_str!("operands.ptx");
    parse_and_assert(vector_add);
}

#[test]
#[allow(non_snake_case)]
fn vectorAdd_kernel64_ptx() {
    let vector_add = include_str!("vectorAdd_kernel64.ptx");
    parse_and_assert(vector_add)
}

#[test]
#[allow(non_snake_case)]
fn _Z9vectorAddPKfS0_Pfi_ptx(){
    let vector_add = include_str!("_Z9vectorAddPKfS0_Pfi.ptx");
    parse_and_assert(vector_add)
}

#[test]
#[allow(non_snake_case)]
fn vectorAdd_11_ptx() {
    let vector_add = include_str!("vectorAdd_11.ptx");
    parse_and_assert(vector_add)
}
