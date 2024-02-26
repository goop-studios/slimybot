use crate::{ApplicationContext, Error};
use poise::{
    serenity_prelude::{Role, RoleId},
    CreateReply,
};

/// Set autorole role.
#[poise::command(
    slash_command,
    required_permissions = "ADMINISTRATOR",
    category = "Roles"
)]
pub async fn set_autorole(
    ctx: ApplicationContext<'_>,
    #[description = "Role that will be applied on member join."] role: Role,
) -> Result<(), Error> {
    let role = {
        let mut data = ctx.data().autorole.lock().expect("Poisoned lock.");
        data.role = Some(role.id.get());
        data.role
    };

    let role = if let Some(role) = role {
        role
    } else {
        panic!("Can't find role.");
    };

    let role_name = if let Some(role_name) = RoleId::new(role).to_role_cached(ctx) {
        role_name
    } else {
        panic!("Failed to find role");
    };

    let reply = CreateReply::default()
        .content(format!("Set the autorole to {role_name}"))
        .ephemeral(true);

    ctx.send(reply).await?;
    Ok(())
}
/// Toggle whether autorole should be on or off.
#[poise::command(
    slash_command,
    required_permissions = "ADMINISTRATOR",
    category = "Roles"
)]
pub async fn toggle_autorole(ctx: ApplicationContext<'_>) -> Result<(), Error> {
    let enabled = {
        let mut data = ctx.data().autorole.lock().expect("Poisoned lock.");
        data.enabled = !data.enabled;
        data.enabled
    };

    let reply = CreateReply::default()
        .content(format!("Set autorole to {enabled}"))
        .ephemeral(true);
    ctx.send(reply).await?;
    Ok(())
}
