//! The ban moderation command.

use serenity::{
	http::CacheHttp,
	model::prelude::application_command::ApplicationCommandInteraction,
	prelude::Context,
};

use crate::{
	aegis::Aegis,
	bot::core::moderation::ModerationParameters,
};

/// Bans a member from a guild, while also cleaning up their messages.
///
/// # Panics
///
/// This function may panic if the command is invoked not within a guild (a.k.a.
/// from the bot's DMs).
///
/// # Errors
///
/// This function propagates errors from
/// [`serenity::model::guild::guild_id::GuildId::ban_with_reason`].
pub async fn ban(
	params: &ModerationParameters,
	context: &Context,
	interaction: &ApplicationCommandInteraction,
) -> Aegis<()> {
	interaction
		.guild_id
		.unwrap()
		.ban_with_reason(context.http(), params.user, 7, &params.reason)
		.await?;
	Ok(())
}
