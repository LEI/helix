// Trusted workspace: ensure it is safe to clone a repo and read source code
// without any unauthorized code execution.

// VS Code features:
// - Tasks
// - Debugging
// - Workspace settings
// - Extensions

// User Guide: https://code.visualstudio.com/docs/editor/workspace-trust
// Extensions: https://code.visualstudio.com/api/extension-guides/workspace-trust

// https://github.com/microsoft/vscode/blob/main/src/vs/workbench/services/workspaces/test/common/workspaceTrust.test.ts

use std::path::{Path, PathBuf};

// Workspace trust (prompt when a new folder is open):
// - Do you trust the authors of the files in this folder?
#[derive(Debug)]
pub struct WorkspaceCommand {
    pub option: WorkspacePrompt,
    pub path: PathBuf,
    pub description: String,
}

impl From<&WorkspaceCommand> for String {
    fn from(command: &WorkspaceCommand) -> Self {
        let option = command.option;
        match option {
            WorkspacePrompt::Trust => format!("{:?}: {:?}", option, command.path),
            WorkspacePrompt::TrustParent => format!("{:?}: {:?}", option, command.path.parent()),
            WorkspacePrompt::DontTrust => format!("{:?}", option),
        }
    }
}

impl WorkspaceCommand {
    pub fn new(option: &WorkspacePrompt, path: &Path) -> Self {
        WorkspaceCommand {
            path: path.to_path_buf(),
            option: *option,
            description: String::from(match option {
                WorkspacePrompt::Trust => "Trust folder and enable all features",
                WorkspacePrompt::TrustParent => "Trust parent folder and enable all features",
                WorkspacePrompt::DontTrust => "Browse folder in restricted mode",
            }),
        }
    }

    pub fn execute(&self) {
        log::info!("TODO {:?}: {:?}", self.option, self.path);
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorkspacePrompt {
    // Trust folder and enable all features
    Trust,
    // Trust the authors of all files in the parent folder
    TrustParent,
    // Browse folder in restricted mode
    DontTrust,
}

impl From<WorkspacePrompt> for String {
    fn from(option: WorkspacePrompt) -> Self {
        let str = match option {
            WorkspacePrompt::Trust => "Trust",
            WorkspacePrompt::TrustParent => "Trust parent",
            WorkspacePrompt::DontTrust => "Don't trust",
        };

        String::from(str)
    }
}

// Untrusted files (prompt when a file located outside of a trusted folder is open)
// - Do you trust the authors of these files?
// Trying to open untrusted files in a workspace which is trusted
// Remember my decision for all workspaces
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UntrustedFilePrompt {
    // Always open untrusted files
    Open,
    // Open in a new window (unsupported?)
    OpenRestricted,
    // Abort the action
    Cancel,
}

// Workspace mode
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorkspaceStatus {
    // Enable all features
    Trusted,
    // Safe code browsing
    Restricted,
}

impl Default for WorkspaceStatus {
    fn default() -> Self {
        // TODO: Trusted if config.security.enable == false
        WorkspaceStatus::Restricted
    }
}

impl From<bool> for WorkspaceStatus {
    fn from(trusted: bool) -> Self {
        match trusted {
            true => WorkspaceStatus::Trusted,
            false => WorkspaceStatus::Restricted,
        }
    }
}

impl From<WorkspaceStatus> for bool {
    fn from(status: WorkspaceStatus) -> bool {
        match status {
            WorkspaceStatus::Trusted => true,
            WorkspaceStatus::Restricted => false,
        }
    }
}

// // Workspace directory
// pub struct Workspace {
//     // pub host: Option<String>,
//     pub path: PathBuf,
//     // pub trusted: bool,
//     pub status: WorkspaceStatus,
// }

// Controls when the startup prompt to trust a workspace is shown.
enum StartupPrompt {
    // Ask for trust every time an untrusted workspace is opened.
    Always,
    // Ask for trust the first time an untrusted workspace is opened.
    Once,
    // Do not ask for trust when an untrusted workspace is opened.
    Never,
}

// Controls how to handle opening untrusted files in a trusted workspace.
// This setting also applies to opening files in an empty window which is trusted via `Configuration.empty_window`.
enum UntrustedFiles {
    // Ask how to handle untrusted files for each workspace. Once untrusted files are introduced to a trusted workspace, you will not be prompted again.
    Prompt,
    // Always allow untrusted files to be introduced to a trusted workspace without prompting.
    Open,
    // Always open untrusted files in a separate window in restricted mode without prompting.
    NewWindow,
}

// Controls when the restricted mode banner is shown.
// - always: Show the banner every time an untrusted workspace is open.
// - untilDismissed: Show the banner when an untrusted workspace is opened until dismissed.
// - never: Do not show the banner when an untrusted workspace is open.
enum Banner {
    // Ask every time an untrusted workspace is open.
    Always,
    // Ask when an untrusted workspace is open until dismissed.
    UntilDismissed,
    // Never ask when an untrusted workspace is open.
    Never,
}

// Workspace trust settings
struct Configuration {
    //enabled: bool,
    //settings: Settings,

    // Whether to show the Workspace Trust dialog on startup.
    // Default is to only show once per distinct folder or workspace.
    startup_prompt: StartupPrompt,

    // Whether to always trust an empty window (no open folder).
    // Default is true.
    empty_window: bool,

    // Controls how to handle loose files in a workspace.
    // Default is to prompt.
    untrusted_files: UntrustedFiles,

    // Controls when the Restricted Mode banner is displayed.
    banner: Banner,
}

impl Default for Configuration {
    fn default() -> Self {
        Self {
            // enabled: true,
            startup_prompt: StartupPrompt::Once,
            empty_window: true,
            untrusted_files: UntrustedFiles::Prompt,
            banner: Banner::UntilDismissed,
        }
    }
}

// const TODO: Configuration = Configuration::default();

// fn add_trusted(path: String) {
//     // TODO.trusted.push(path);
// }

// fn is_trusted(path: &PathBuf) -> bool {
//     let parent = path.parent();
//     if parent.is_none() {
//         // scratch buffer?
//         return true;
//     }
//     let directory = parent.unwrap().to_str().unwrap();
//     let is_trusted = trusted.iter().any(|dir| directory.starts_with(dir));
//     log::info!("--- IS TRUSTED? {:?}", is_trusted);
//     is_trusted
// }

// pub fn get_workspace(path: &PathBuf) -> Workspace {
//     Workspace {
//         path: path.to_owned(),
//         status: get_status(&path),
//     }
// }

// Restricted Mode is intended for safe code browsing.
// Trust this folder to enable all features.
// - Manage: open workspace security configuration picker
// - Learn More: link to documentation

// fn prompt_trust_folder(path: &PathBuf) -> Option<Banner> {
//     if is_trusted(&path) {
//         return None;
//     }
//     Some(Banner::UntilDismissed)
// }

// fn workspace_trust() -> std::io::Result<()> {
//     let stdout = std::io::stdout();
//     let mut stdout = stdout.lock();
//     writeln!(stdout, "{}", "Workspace trust enabled")?;

//     Ok(())
// }

#[cfg(test)]
mod workspace {
    // use super::*;

    // #[test]
    // fn trust_enabled() {
    //     let default_config = Configuration::default();
    //     assert_eq!(default_config.enabled, true);
    // }

    // #[test]
    // fn trust_disabled() {
    //     let default_config = Configuration::default();
    //     // TODO: disable from user config
    //     assert_eq!(default_config.enabled, false);
    // }

    // #[test]
    // fn empty_trusted() {
    //     let default_config = Configuration::default();
    //     // TODO: open scratch buffer
    //     assert_eq!(default_config.enabled, true);
    // }

    // #[test]
    // fn empty_untrusted() {
    //     let mut default_config = Configuration::default();
    //     default_config.empty_window = false;
    //     // TODO: open scratch buffer
    //     assert_eq!(default_config.enabled, false);
    // }

    // #[test]
    // fn empty_trusted_open_trusted_file() {
    //     let default_config = Configuration::default();
    //     // TODO: init trusted /Folder and open /Folder/file.txt
    //     assert_eq!(default_config.enabled, true);
    // }

    // #[test]
    // fn empty_trusted_open_untrusted_file() {
    //     let default_config = Configuration::default();
    //     // TODO: directly open /Folder/file.txt
    //     assert_eq!(default_config.enabled, false);
    // }

    // #[test]
    // fn test_is_trusted() {
    //     assert_eq!(is_trusted(&PathBuf::from("/tmp/trusted")), true);
    // }
}
