use devout::{Tag, log};
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

const INFO: Tag = Tag::new("Info");

#[wasm_bindgen]
pub fn main() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();

    // Print out code from regular example.
    log!(INFO, "User→Name      whoami::realname():    {}", whoami::realname());
    log!(INFO, "User→Username  whoami::username():    {}", whoami::username());
    log!(INFO, "Host→Name      whoami::devicename():  {}", whoami::devicename());
    log!(INFO, "Host→Hostname  whoami::hostname():    {}", whoami::hostname());
    log!(INFO, "Platform       whoami::platform():    {}", whoami::platform());
    log!(INFO, "OS Distro      whoami::distro():      {}", whoami::distro());
    log!(INFO, "Desktop Env.   whoami::desktop_env(): {}", whoami::desktop_env());
}
