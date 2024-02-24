use crate::{ApplicationContext, Data, Error};
use poise::serenity_prelude::{self as serenity, CacheHttp, Member, Role};

#[poise::command(slash_command, required_permissions = "ADMINISTRATOR")]
pub async fn set_autorole(ctx: ApplicationContext<'_>, role: Role) -> Result<(), Error> {
    let role = {
        let mut data = ctx.data().autorole.lock().expect("Poisoned lock.");
        data.role = Some(role.id.get());
        data.role
    };

    let reply = format!("Set the autorole to: {}", role.unwrap());
    ctx.reply(reply).await?;
    Ok(())
}

#[poise::command(slash_command, required_permissions = "ADMINISTRATOR")]
pub async fn toggle_autorole(ctx: ApplicationContext<'_>) -> Result<(), Error> {
    let enabled = {
        let mut data = ctx.data().autorole.lock().expect("Poisoned lock.");
        data.enabled = !data.enabled;
        data.enabled
    };

    let reply = format!("Set automod to {enabled}");
    ctx.reply(reply).await?;
    Ok(())
}

pub async fn set_role(
    ctx: &serenity::Context,
    data: &Data,
    new_member: &Member,
) -> Result<(), Error> {
    if data
        .autorole
        .try_lock()
        .expect("poisoned lock.").enabled
    {
        let role = data
            .autorole
            .try_lock()
            .expect("poisoned lock.")
            .role
            .expect("Welcome channel not set.");
        let role_id = serenity::RoleId::new(role);
        new_member.add_role(&ctx.http(), role_id).await?;
    }
    Ok(())
}
