use crate::constant;

pub struct EnvTestContext {
    original_value: Option<String>,
}

impl Drop for EnvTestContext {
    fn drop(&mut self) {
        if self.original_value.is_none() {
            return;
        }

        println!(
            "{} env var was set. Resetting it...",
            constant::template_manager::HOME_ENV_VAR
        );
        set_env_var(
            constant::template_manager::HOME_ENV_VAR,
            self.original_value.as_mut().unwrap(),
        );
        println!("Reset!");

        println!("Test context dropped");
    }
}

pub fn create_env_test_context() -> EnvTestContext {
    let ctx = match std::env::var(constant::template_manager::HOME_ENV_VAR) {
        Ok(result) => EnvTestContext {
            original_value: Some(result),
        },
        Err(_) => EnvTestContext {
            original_value: None,
        },
    };

    println!("Test context created");

    if ctx.original_value.is_some() {
        println!(
            "{} env var is set. Removing it...",
            constant::template_manager::HOME_ENV_VAR
        );
        remove_env_var(constant::template_manager::HOME_ENV_VAR);
        println!("Removed!");
    }

    ctx
}

pub fn remove_env_var<T: AsRef<std::ffi::OsStr>>(name: T) {
    unsafe {
        std::env::remove_var(name);
    }
}

pub fn set_env_var<T: AsRef<std::ffi::OsStr>, V: AsRef<std::ffi::OsStr>>(
    name: T,
    value: V,
) {
    unsafe {
        std::env::set_var(name, value);
    }
}
