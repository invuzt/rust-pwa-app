use wasm_bindgen::prelude::*;

// Kita "meminjam" fungsi window.alert dari JavaScript
#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

// Fungsi ini yang akan dipanggil saat PWA dibuka
#[wasm_bindgen(start)]
pub fn run() {
    alert("Halo! Ini adalah pesan dari Rust Engine 🦀");
}

