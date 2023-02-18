use super::command::SlashCommand;

struct Ping;

impl SlashCommand for Ping {
    fn register(
        command: &mut serenity::builder::CreateApplicationCommand,
    ) -> &mut serenity::builder::CreateApplicationCommand {
        command
            .name("ping")
            .description("A simple command to ping the bot")
    }

    fn run(
        _options: &[serenity::model::prelude::interaction::application_command::CommandDataOption],
    ) -> String {
        "Pong!".to_string()
    }
}
