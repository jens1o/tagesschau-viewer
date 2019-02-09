extern crate rss;

use std::env;
use std::fmt;
use std::process::{Command, Stdio};

const BASE_URL: &str = "https://www.tagesschau.de/export/video-podcast/webxl/";

const SUFFIX_URL: &str = "_https/";

enum EpisodeType {
    Tagesthemen,
    Tagesschau,
}

impl fmt::Display for EpisodeType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                EpisodeType::Tagesschau => "tagesschau",
                EpisodeType::Tagesthemen => "tagesthemen",
            }
        )
    }
}

impl EpisodeType {
    fn get_channel_url(&self) -> String {
        format!("{}{}{}", BASE_URL, self.to_string(), SUFFIX_URL)
    }
}

fn main() -> Result<(), rss::Error> {
    let episode_type: EpisodeType = env::args()
        .nth(1)
        .and_then(|x| {
            if x == "tt" || x == "tagesthemen" {
                Some(EpisodeType::Tagesthemen)
            } else {
                None
            }
        })
        .unwrap_or(EpisodeType::Tagesschau);

    println!("Playing latest {} episode.", episode_type.to_string());

    let channel = rss::Channel::from_url(&episode_type.get_channel_url())?;

    let video_url = channel
        .items()
        .first()
        .expect("No video item in RSS feed!")
        .enclosure()
        .expect("Video item has no video data?!")
        .url();

    println!("Playing video url: {}", video_url);

    let exit_code = play_video(video_url);

    if exit_code != 0 {
        eprintln!();
        eprintln!("vlc didn't finished with non-zero error code!");
    } else {
        println!("Finished. Bye…");
    }

    Ok(())
}

fn play_video(source: &str) -> i32 {
    const PROGRAM_NAME: &str = "vlc";

    Command::new(PROGRAM_NAME)
        .arg("--play-and-exit")
        .args(&["--verbose", "0"])
        .arg("-f")
        .arg(source)
        .stdout(Stdio::null()) // pipe everything into void
        .spawn()
        .expect("Failed to spawn vlc to play the video!")
        .wait()
        .unwrap()
        .code()
        .unwrap()
}
