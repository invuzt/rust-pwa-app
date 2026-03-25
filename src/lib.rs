use wasm_bindgen::prelude::*;
use web_sys::window;

#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    // Ambil object window browser
    let window = window().expect("Gak ada window global");
    let document = window.document().expect("Gak bisa ambil document");
    
    // Cari elemen h1 dengan id="status"
    let val = document.get_element_by_id("status")
        .expect("Elemen id='status' gak ketemu");
    
    // Ubah isi teksnya lewat Rust!
    val.set_inner_html("Mantap! Teks ini dikirim langsung dari Rust Engine 🦀");

    Ok(())
}

