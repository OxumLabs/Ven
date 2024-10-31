use crate::{
    archs::{l32::lfor32, l64::lfor64, mac32::m32, mac64::m64, win32::w32, win64::w64},
    types::Types,
};
pub fn mkasm(tokens: Vec<Types>, target: String) -> String {
    let mut asm_code = String::new();
    match target.as_str() {
        "LM" => asm_code = lfor64(tokens),
        "LHM" => asm_code = lfor32(tokens),
        "WM" => asm_code = w64(tokens),
        "WHM" => asm_code = w32(tokens),
        "MM" => asm_code = m64(tokens),
        "MHM" => asm_code = m32(tokens),
        _ => eprintln!("Unsupported target: {}", target),
    }
    asm_code
}
