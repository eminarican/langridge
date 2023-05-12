use deepl::{DeepLApi, Lang};
use serenity::async_trait;

use serenity::prelude::*;
use serenity::model::prelude::*;

use settings::{Room, Settings};

mod settings;

struct Handler {
    deepl: DeepLApi,
    settings: Settings,
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.author.bot || msg.guild_id.is_none() {
            return;
        }

        for group in self.settings.groups.iter() {
            for room in group {
                if room.id != msg.channel_id.0 {
                    continue
                }

                for target in group {
                    if room.id == target.id {
                        continue
                    }

                    self.send_translated(
                        &ctx, &msg, &msg.guild_id.unwrap(), target
                    ).await;
                }
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

impl Handler {
    async fn send_translated(&self, ctx: &Context, msg: &Message, guild: &GuildId, room: &Room) {
        let translated = self.deepl
            .translate_text(
                &msg.content,
                Lang::try_from(&room.lang).expect("invalid room lang")
            )
            .await.expect("couldn't translate text").to_string();

        let webhook = Webhook::from_url(&ctx.http, &room.webhook)
            .await.expect("couldn't create webhook");

        let member = guild.member(&ctx.http, &msg.author.id)
            .await.expect("couldn't get guild member");

        let name = member.display_name();
        let avatar = member.avatar_url().unwrap_or_else(|| msg.author.avatar_url().unwrap());

        if !msg.content.is_empty() {
            webhook.execute(&ctx.http, false, |w| {
                w.username(&name).avatar_url(&avatar).content(&translated)
            }).await.expect("couldn't send message content");
        }

        for sticker in &msg.sticker_items {
            webhook.execute(&ctx.http, false, |w| {
                w.username(&name).avatar_url(&avatar)
                    .content(&sticker.image_url().expect("couldn't get sticker url"))
            }).await.expect("couldn't send message attachment");
        }

        for attachment in &msg.attachments {
            webhook.execute(&ctx.http, false, |w| {
                w.username(&name).avatar_url(&avatar).content(&attachment.url)
            }).await.expect("couldn't send message attachment");
        }
    }
}

#[tokio::main]
async fn main() {
    let settings = Settings::new().expect("couldn't read settings");

    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

    let deepl = DeepLApi::with(&settings.secrets.deepl).new();

    let mut client = Client::builder(&settings.secrets.discord, intents)
        .event_handler(Handler { deepl, settings })
        .await.expect("couldn't create client");

    client.start()
        .await.expect("couldn't start client");
}
