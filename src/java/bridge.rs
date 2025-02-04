use jni::objects::{JClass, JString};
use jni::sys::jstring;
use jni::JNIEnv;

/// Example JNI function to return a simple string
#[no_mangle]
pub extern "system" fn Java_sc_whis_sh_scripture_HelloWorld_hello(
    env: JNIEnv,
    _class: JClass,
    input: JString,
) -> jstring {
    let input: String = env.get_string(input).expect("Couldn't get Java string!").into();
    let output = format!("Hello, {}!", input);
    env.new_string(output).expect("Couldn't create Java string!").into_raw()
}
