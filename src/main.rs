use serenity::{
	all::{Client, Context, EventHandler, GatewayIntents, GuildId, Interaction, Ready},
	async_trait,
};

mod commands;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
	async fn ready(&self, ctx: Context, ready: Ready) {
		println!("{} is online!", ready.user.name);

		let guild_id = GuildId::new(
			dotenv::var("GUILD_ID")
				.expect("Expected a guild id in the environment")
				.parse::<u64>()
				.expect("Guild id is not an integer"),
		);

		let commands = guild_id
			.set_commands(&ctx.http, vec![commands::ping::register()])
			.await
			.expect("Failed to create commands");

		println!(
			"Registered the following commands: {}",
			commands
				.into_iter()
				.map(|c| c.name)
				.collect::<Vec<String>>()
				.join(", ")
		)
	}

	async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
		if let Interaction::Command(cmd) = interaction
			&& cmd.data.name.as_str() == "ping"
		{
			commands::ping::run(&ctx, &cmd).await
		}
	}
}

#[tokio::main]
async fn main() {
	let token = dotenv::var("DISCORD_TOKEN").expect("Expected a token in the environment");
	let intents = GatewayIntents::GUILD_MESSAGES;

	let mut client = Client::builder(&token, intents)
		.event_handler(Handler)
		.await
		.expect("Error creating client");

	if let Err(why) = client.start().await {
		println!("Client error: {why:?}")
	}
}
