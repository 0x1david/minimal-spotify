extern crate dotenv;

pub fn scope() -> String {
        "user-modify-playback-state \
        user-read-playback-state \
        user-read-currently-playing \
        streaming \
        playlist-read-private \
        playlist-read-collaborative \
        playlist-modify-private \
        playlist-modify-public \
        user-follow-modify \
        user-follow-read \
        user-read-playback-position \
        user-top-read \
        user-read-recently-played \
        user-library-modify \
        user-library-read".to_string()
}

pub fn auth_url() -> String {
    "https://accounts.spotify.com/authorize".to_string()
}

