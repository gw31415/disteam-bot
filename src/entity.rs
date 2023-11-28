use poise::serenity_prelude::{self as serenity, Color, UserId};

/// Error (to be resolved during execution)
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Serenity(#[from] serenity::Error),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

impl Error {
    pub fn get_embed(
        &self,
    ) -> impl FnOnce(&mut serenity::CreateEmbed) -> &mut serenity::CreateEmbed {
        let msg = self.to_string();
        |embed| embed.title("Error").color(Color::RED).description(msg)
    }
}

/// 管理スタイル
pub enum ManagementStyle {
    /// チャンネル作成権限をチーム全員に付与
    Repub,
    /// グループ代表者にチャンネル作成権限を付与
    Feudal { manager: UserId },
    /// チャンネル作成権限を付与しない。
    Monarchy,
}
