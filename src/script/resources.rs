use anyhow::anyhow;
use bevy::asset::{AssetLoader, LoadContext, LoadedAsset};
use bevy::prelude::*;
use bevy::reflect::TypeUuid;
use bevy::utils::{BoxedFuture, HashMap};
use sscanf::scanf;

/// A single command which formatted as `.<command> <arg1> <arg2> ...`
#[derive(Debug, Default, Clone)]
pub struct WgsCommand {
    pub command: String,
    pub args: Vec<String>,
}

/// Collection of all `.lable <label>` in the script.
/// Store `(<lable>, <line>)` pairs.
#[derive(Debug, Deref, DerefMut, Default, Clone)]
pub struct WgsLabelMap(pub HashMap<String, usize>);

/// A parsed `.wgs` script file which can be executed directly by `WgsVirtualMachine`.
/// It contains a collection of `WgsCommand` and `WgsLabelMap`.
#[derive(Debug, Default, Clone)]
pub struct WgsParsedScript {
    pub commands: Vec<WgsCommand>,
    pub label_map: WgsLabelMap,
}

/// A `.wgs` script file which be managed by bevy`AssetServer`.
/// - All `.wgs` script files will be loaded and stored as `Bytecode(Vec<u8>)` when the game process initialized.
/// - The `.wgs` script file will be parsed when this asset is first loaded and executed by `WgsVirtualMachine`.
/// After that, the handle of this file will refer to the parsed `WgsParsedScript`.
#[derive(Debug, Default, Clone, Deref, DerefMut, TypeUuid)]
#[uuid = "C5BECE68-F078-CBD6-C541-FAC2174C1070"]
pub struct WgsScript(pub WgsParsedScript);

impl WgsScript {
    pub fn load(bytes: &[u8]) -> anyhow::Result<WgsScript> {
        if bytes.is_empty() {
            Err(anyhow!(
                "Loading an empty script file! Maybe check the `assets/script` folder?"
            ))
        } else {
            let mut script = WgsScript::default();
            for line in String::from_utf8(bytes.to_vec()).unwrap().lines() {
                // No argument command
                if let Ok(command) = scanf!(line, ".{}", String) {
                    script.0.commands.push(WgsCommand {
                        command,
                        args: vec![],
                    });
                }
                // With argument command
                else if let Ok((command, args)) = scanf!(line, ".{} {}", String, String) {
                    script.0.commands.push(WgsCommand {
                        command: command.clone(),
                        args: args.split_whitespace().map(|s| s.to_string()).collect(),
                    });
                    // Special case: `.label <label>`
                    if command == String::from("label") {
                        let (_, label) = scanf!(line, ".{} {}", String, String).unwrap();
                        script
                            .0
                            .label_map
                            .0
                            .insert(label, script.0.commands.len() - 1);
                    }
                }
                // Parse failed
                else {
                    return Err(anyhow!("Parse failed: {}", line));
                }
            }
            Ok(script)
        }
    }
}

/// The loader of `.wgs` script file.
#[derive(Debug, Default)]
pub struct WgsScriptLoader;

impl AssetLoader for WgsScriptLoader {
    // load asset from file
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<(), anyhow::Error>> {
        Box::pin(async move {
            let script = WgsScript::load(bytes)?;
            load_context.set_default_asset(LoadedAsset::new(script));
            Ok(())
        })
    }

    // file extensions
    fn extensions(&self) -> &[&str] {
        &["wgs"]
    }
}