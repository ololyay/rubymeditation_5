#![crate_type = "dylib"]

extern crate libc;
pub use libc::types::os::arch::c95::{c_char, c_long};
pub use libc::types::common::c95::c_void;

extern fn app_controller(env: *mut libc::c_void) -> *mut libc::c_void {
  unsafe {
    let response = rb_ary_new(); // response => []

    let status = rb_int2big(200);

    rb_ary_push(response, status); // response => [200]

    let headers = rb_hash_new();

    rb_ary_push(response, headers); // response => [200, {}]

    let text = "<h1>RUST WEB APP</h1>";

    let body = rb_str_new(text.to_c_str().as_ptr(), text.len().to_i64().unwrap());
    let body_ary = rb_ary_new();
    rb_ary_push(body_ary, body); // ["<h1>RUST WEB APP</h1>"]

    rb_ary_push(response, body_ary); // response => [200, {}, ["<h1>RUST WEB APP</h1>"]]

    return response;
  }
}

#[link(name = "ruby")]
extern {
  // define ruby module
  fn rb_define_module(name: *const libc::c_char) -> *mut libc::c_void;

  // define singleton method for object
  fn rb_define_singleton_method(
    klass: *mut libc::c_void,
    name: *const libc::c_char,
    callback: extern fn(*mut libc::c_void) -> *mut libc::c_void,
    argc: libc::c_int
  );

  // create empty array
  fn rb_ary_new() -> *mut libc::c_void;

  // push value to array
  fn rb_ary_push(ary: *mut libc::c_void, value: *mut libc::c_void);

  // create empty hash
  fn rb_hash_new() -> *mut libc::c_void;

  // create string
  fn rb_str_new(text: *const libc::c_char, length: libc::c_long) -> *mut libc::c_void;

  // convert simple int to Fixnum
  fn rb_int2big(value: int) -> *mut libc::c_void;
}

#[no_mangle]
pub extern fn Init_librack_app_rust() {
  let module = unsafe { rb_define_module("RackAppRust".to_c_str().as_ptr()) }; // create module RackAppRust
  unsafe { rb_define_singleton_method(module, "call".to_c_str().as_ptr(), app_controller, 1) }; // 'def self.call(env)' in module RackAppRust
}