use zed_extension_api::{self as zed, Result, serde_json::json};

const BIN_NAME: &'static str = "dhall-lsp-server";

#[derive(Default)]
struct DhallExtension {}

impl zed::Extension for DhallExtension {
    fn new() -> Self
    where
        Self: Sized,
    {
        Self::default()
    }

    fn language_server_command(
        &mut self,
        _language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        Ok(zed::Command {
            command: worktree
                .which(BIN_NAME)
                .ok_or_else(|| format!("{} not found", BIN_NAME))?,
            args: vec![],
            env: Default::default(),
        })
    }

    fn language_server_workspace_configuration(
        &mut self,
        _language_server_id: &zed::LanguageServerId,
        _worktree: &zed::Worktree,
    ) -> Result<Option<zed::serde_json::Value>> {
        Ok(Some(json!({
            "vscode-dhall-lsp-server": {},
        })))
    }

    // fn language_server_initialization_options(
    //     &mut self,
    //     _language_server_id: &zed::LanguageServerId,
    //     _worktree: &zed::Worktree,
    // ) -> Result<Option<zed::serde_json::Value>> {
    //     Ok(Some(json!({
    //         "vscode-dhall-lsp-server": {},
    //     })))
    // }
}

zed::register_extension!(DhallExtension);
