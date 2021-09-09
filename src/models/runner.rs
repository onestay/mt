pub struct Runner {
    display_name: String,
    country: String,
    social_media: Vec<SocialMedia>
}

pub enum SocialMedia {
    Twitter(String),
    Twitch(String),
    Youtube(String)
}