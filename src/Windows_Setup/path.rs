use std::ffi::OsStr;
use std::io::Error;
use std::os::windows::ffi::OsStrExt;
use winapi::um::winreg::{HKEY_LOCAL_MACHINE, RegOpenKeyExW, RegSetValueExW, RegCloseKey};
use winapi::um::winnt::{KEY_WRITE, REG_EXPAND_SZ};
use winapi::um::winuser::{SendMessageW, HWND_BROADCAST, WM_SETTINGCHANGE};
use winapi::shared::ntdef::NULL;
use winapi::shared::minwindef::HKEY;

fn set_system_environment_variable(name: &str, value: &str) -> Result<(), Error> {
    unsafe {
        // Open the registry key
        let mut key: HKEY = NULL as HKEY;
        let key_path = OsStr::new(r"SYSTEM\CurrentControlSet\Control\Session Manager\Environment")
            .encode_wide()
            .chain(std::iter::once(0))
            .collect::<Vec<u16>>();
        let name_wide: Vec<u16> = OsStr::new(name).encode_wide().chain(std::iter::once(0)).collect();
        let value_wide: Vec<u16> = OsStr::new(value).encode_wide().chain(std::iter::once(0)).collect();

        let result = RegOpenKeyExW(
            HKEY_LOCAL_MACHINE,
            key_path.as_ptr(),
            0,
            KEY_WRITE,
            &mut key,
        );
        if result != 0 {
            return Err(Error::last_os_error());
        }

        // Set the environment variable
        let result = RegSetValueExW(
            key,
            name_wide.as_ptr(),
            0,
            REG_EXPAND_SZ,
            value_wide.as_ptr() as _,
            (value_wide.len() * 2) as u32,
        );
        if result != 0 {
            RegCloseKey(key); // Ensure key is closed on error
            return Err(Error::last_os_error());
        }

        // Close the registry key
        RegCloseKey(key);

        // Notify the system of the change
        SendMessageW(HWND_BROADCAST, WM_SETTINGCHANGE, 0, 0);

        Ok(())
    }
}

fn main() {
    // Define the environment variable name and value
    let env_var_name = "CODERUSH_PATH";
    let env_var_value = r"C:\Program Files\coderush";

    // Set the environment variable for the current process
    std::env::set_var(env_var_name, env_var_value);
    println!("Environment variable {} set to {}", env_var_name, env_var_value);

    // Set the system-wide environment variable
    match set_system_environment_variable(env_var_name, env_var_value) {
        Ok(_) => println!("System environment variable {} set to {}", env_var_name, env_var_value),
        Err(e) => eprintln!("Failed to set system environment variable: {}", e),
    }
}
