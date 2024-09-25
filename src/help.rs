// help.rs
pub fn display_help() {
    println!(
        "Commands:\n\n\
         coderush <filename>.(Any Language Extension) It will return output as well as input.\n\n\
         Attributes:\n\n\
         -c This attribute will change the compiler into the following:\n\n\
         mono        Changes C#'s compiler to Mono.\n\
         dotnet      Changes C#'s compiler to .NET.\n\n\
         --h         This will display help.
--version   This will displays version.
         "
    );
}
