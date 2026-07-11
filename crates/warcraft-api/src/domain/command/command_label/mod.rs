use crate::domain::identity::WarcraftObjectId;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct CommandLabel;

impl CommandLabel {
    pub fn pretty(command_name: WarcraftObjectId) -> String {
        let command_value = command_name.value();
        let stripped = command_value.strip_prefix("Cmd").unwrap_or(command_value);
        if stripped.is_empty() {
            return command_value.to_string();
        }
        stripped.to_string()
    }
}
