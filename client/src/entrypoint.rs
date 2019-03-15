use wasm_bindgen::prelude::*;
use std::panic;


// Called when the wasm module is instantiated
//
#[ wasm_bindgen( start ) ]
//
pub fn main() -> Result<(), JsValue>
{
	panic::set_hook( Box::new( console_error_panic_hook::hook ) );

	// Use `web_sys`'s global `window` function to get a handle on the global window object.
	//
	let window   = web_sys::window()  .expect( "no global `window` exists"        );
	let document = window  .document().expect( "should have a document on window" );
	let _body    = document.body()    .expect( "document should have a body"      );

	// Manufacture the element we're gonna append
	// let val = document.create_element( "div" )?;

	// document.get_element_by_id( "" ).unwrap().append_child( &val )?;

	Ok(())
}

/*
#[wasm_bindgen]
//
pub fn add(a: u32, b: u32) -> u32 {
	a + b
}
*/
