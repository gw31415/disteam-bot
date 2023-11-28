mod entity;
mod util;

use entity::Error;
use poise::{
    serenity_prelude::{self as serenity, Color, User},
    Event,
};

use crate::entity::ManagementStyle;

type Data = ();
type Context<'a> = poise::Context<'a, Data, Error>;

/// チームを作成します。
#[poise::command(slash_command, subcommands("repub", "monarchy", "feudal"))]
async fn mkteam(_ctx: Context<'_>) -> Result<(), Error> {
    Ok(())
}

/// チームを作成します。チャンネル作成権限をチーム全員に付与します。
#[poise::command(slash_command)]
async fn repub(
    ctx: Context<'_>, #[description = "チーム名"] name: String
) -> Result<(), Error> {
    util::mkteam(ctx, name, ManagementStyle::Repub).await
}

/// チームを作成します。チャンネル作成権限を付与しません。
#[poise::command(slash_command)]
async fn monarchy(
    ctx: Context<'_>, #[description = "チーム名"] name: String
) -> Result<(), Error> {
    util::mkteam(ctx, name, ManagementStyle::Monarchy).await
}

/// チームを作成します。代表者にチャンネル作成権限を付与します。
#[poise::command(slash_command)]
async fn feudal(
    ctx: Context<'_>,
    #[description = "チーム名"] name: String,
    #[description = "管理者"] manager: User,
) -> Result<(), Error> {
    util::mkteam(ctx, name, ManagementStyle::Feudal { manager: manager.id }).await
}

#[tokio::main]
async fn main() {
    #[cfg(feature = "dotenv")]
    dotenvy::dotenv().unwrap();

    #[cfg(feature = "logger")]
    env_logger::init();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![mkteam()],
            event_handler: |ctx, event, framework, data| {
                Box::pin(event_handler(ctx, event, framework, data))
            },
            on_error: |err| Box::pin(on_error(err)),
            ..Default::default()
        })
        .intents(serenity::GatewayIntents::non_privileged())
        .token(std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN"))
        .setup(|_ctx, _ready, _framework| Box::pin(async move { Ok(()) }));
    framework.run().await.unwrap();
}

/// 例外処理
async fn on_error(err: poise::FrameworkError<'_, Data, Error>) {
    if let Some(ctx) = err.ctx() {
        use poise::FrameworkError::*;
        let _ = ctx
            .send(|b| match &err {
                Command { error, .. }
                | Setup { error, .. }
                | EventHandler { error, .. }
                | DynamicPrefix { error, .. } => {
                    #[cfg(feature = "logger")]
                    log::error!("{:?}", err);
                    b.embed(error.get_embed())
                }
                _ => {
                    #[cfg(feature = "logger")]
                    {
                        log::error!("{:?}", err);
                    }
                    b.embed(|embed| {
                        embed
                            .color(Color::RED)
                            .title("Error")
                            .description("Unhandled error occured.")
                    })
                }
                .ephemeral(true),
            })
            .await;
    }
}

/// 自動でギルドコマンドを登録する
async fn event_handler(
    ctx: &serenity::Context,
    event: &Event<'_>,
    framework: poise::FrameworkContext<'_, Data, Error>,
    _data: &Data,
) -> Result<(), Error> {
    match event {
        Event::GuildMemberAddition { new_member } => {
            if framework.bot_id == new_member.user.id {
                poise::builtins::register_in_guild(
                    ctx,
                    &framework.options().commands,
                    new_member.guild_id,
                )
                .await?;
            }
        }
        Event::Ready { data_about_bot } => {
            for guild in &data_about_bot.guilds {
                poise::builtins::register_in_guild(ctx, &framework.options().commands, guild.id)
                    .await?;
            }
        }
        _ => (),
    }
    Ok(())
}
