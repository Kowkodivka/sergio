use serenity::{
    builder::CreateApplicationCommand,
    model::prelude::interaction::application_command::CommandDataOption,
};

pub trait SlashCommand {
    fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand;
    fn run(options: &[CommandDataOption]) -> String;
}
