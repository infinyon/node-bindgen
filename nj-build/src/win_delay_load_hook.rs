#[cfg(windows)]
pub fn build(dir: std::path::PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    use std::fs::{File, remove_file};
    use std::io::prelude::*;
    use std::env::current_dir;

    let file_name = "win_delay_load_hook";
    let mut tmp_file = current_dir()?;
    tmp_file.push(&format!("{}.cc", file_name));

    {
        const WIN_DELAY_LOAD_HOOK: &str = r##"
      /*
      * When this file is linked to a DLL, it sets up a delay-load hook that
      * intervenes when the DLL is trying to load the host executable
      * dynamically. Instead of trying to locate the .exe file it'll just
      * return a handle to the process image.
      *
      * This allows compiled addons to work when the host executable is renamed.
      */

      #ifdef _MSC_VER

      #pragma managed(push, off)

      #ifndef WIN32_LEAN_AND_MEAN
      #define WIN32_LEAN_AND_MEAN
      #endif

      #include <windows.h>

      #include <delayimp.h>
      #include <string.h>

      static FARPROC WINAPI load_exe_hook(unsigned int event, DelayLoadInfo* info) {
        HMODULE m;
        if (event != dliNotePreLoadLibrary)
          return NULL;

        if (_stricmp(info->szDll, "node.exe") != 0)
          return NULL;

        m = GetModuleHandle(NULL);
        return (FARPROC) m;
      }

      decltype(__pfnDliNotifyHook2) __pfnDliNotifyHook2 = load_exe_hook;

      #pragma managed(pop)

      #endif
      "##;

        let mut file = File::create(&tmp_file)?;
        file.write_all(WIN_DELAY_LOAD_HOOK.as_bytes())?;
    }

    // Build the `win_delay_load_hook.o` file;
    cc::Build::new()
        .file(&tmp_file)
        .out_dir(&dir)
        .compile(file_name);

    // Remove tmp file once the artifact is built;
    remove_file(&tmp_file)?;

    Ok(())
}
