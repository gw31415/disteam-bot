use anyhow::{anyhow, Context as _};
use poise::serenity_prelude::{
    self as serenity, model::permissions::Permissions, ChannelType, Color, PermissionOverwrite,
};

use crate::entity::{Error, ManagementStyle};

type Data = ();
type Context<'a> = poise::Context<'a, Data, Error>;

/// チームを作成します
pub async fn mkteam(ctx: Context<'_>, name: String, style: ManagementStyle) -> Result<(), Error> {
    // ギルドの取得
    let guild = ctx.guild().context("ギルドが見つかりませんでした。")?;

    // 名前が重複していないかどうか確認する
    if guild.roles.values().any(|role| role.name == name)
        || guild
            .channels(ctx)
            .await?
            .values()
            .any(|ch| ch.kind == ChannelType::Category && ch.name == name)
    {
        return Err(anyhow!("名前 `{name}` は重複しています。").into());
    }

    // ロールの作成
    let role = guild.create_role(ctx, |r| r.name(&name)).await?;

    // カテゴリの作成
    let category = {
        // 権限設定のセットアップ
        let permissions_overwrite = {
            // 閲覧権限
            let view = Permissions::VIEW_CHANNEL | Permissions::CONNECT;

            let mut permissions = vec![
                // チームは閲覧許可
                PermissionOverwrite {
                    kind: serenity::PermissionOverwriteType::Role(role.id),
                    allow: view,
                    deny: Permissions::empty(),
                },
                // @everyoneは閲覧禁止
                PermissionOverwrite {
                    kind: serenity::PermissionOverwriteType::Role({
                        // @everyone のロール
                        serenity::RoleId::from(guild.id.0)
                    }),
                    deny: view,
                    allow: Permissions::empty(),
                },
            ];

            match style {
                // Repub の場合はチャンネルの作成権限を与える
                ManagementStyle::Repub => {
                    permissions.push(PermissionOverwrite {
                        kind: serenity::PermissionOverwriteType::Role(role.id),
                        allow: Permissions::MANAGE_CHANNELS,
                        deny: Permissions::empty(),
                    });
                }
                ManagementStyle::Feudal { manager } => {
                    permissions.push(PermissionOverwrite {
                        kind: serenity::PermissionOverwriteType::Member(manager),
                        allow: Permissions::MANAGE_CHANNELS,
                        deny: Permissions::empty(),
                    });
                }
                ManagementStyle::Monarchy => (),
            }

            permissions
        };

        // カテゴリの作成
        guild
            .create_channel(ctx, |c| {
                c.name(&name)
                    .kind(ChannelType::Category)
                    .permissions(permissions_overwrite)
            })
            .await?
    };

    // vcを1つ作成する
    guild
        .create_channel(ctx, |c| {
            c.name("会議室").category(category).kind(ChannelType::Voice)
        })
        .await?;
    ctx.send(|b| {
        b.embed(|embed| {
            embed
                .title("OK")
                .color(Color::DARK_GREEN)
                .description(format!("チーム `{name}` を作成しました。"))
        })
    })
    .await?;
    Ok(())
}
