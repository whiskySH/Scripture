use j4rs::{Instance, Jvm};

/// Loads a Java class and calls a method
pub fn call_java_method() -> j4rs::Result<()> {
    let jvm = Jvm::attach_current_thread()?;
    let instance = Instance::new(&jvm, "java.lang.String", vec!["Hello from Rust"])?;
    
    let result: String = jvm.invoke(&instance, "toUpperCase", &[])?;
    println!("Java method result: {}", result);

    Ok(())
}
