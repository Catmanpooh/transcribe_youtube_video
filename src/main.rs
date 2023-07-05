use async_openai::{
    types::{CreateTranscriptionRequest,AudioInput, AudioResponseFormat},
    Client
};
use youtube_dl::YoutubeDl;
use std::{fs::File, io::Write};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = Client::new();

    let url: &str = "https://www.youtube.com/watch?v=_x6iumj0Tmo";

    let output = YoutubeDl::new(url)
        .extract_audio(true)
        .download(true)
        .format("m4a")
        .output_template("%(title)s.%(ext)s")
        .output_directory("./audio/")
        .run_async()
        .await?;

    let title = output
        .into_single_video()
        .expect("ERROR: error getting video title")
        .title;


    let audio_path = AudioInput {
        path: std::path::PathBuf::from(format!("./audio/{}.m4a", title))
    };

    let request = CreateTranscriptionRequest {
        file: audio_path,
        model: "whisper-1".to_string(),
        prompt: None, 
        response_format: Some(AudioResponseFormat::Json), 
        temperature: None,
        language: Some("en".to_string()),
    };

    let response = client
        .audio()
        .transcribe(request)
        .await
        .expect("Error: issuse while trying to gert response");

    let title: String = title
        .to_lowercase()
        .chars().filter(|c| !c.is_whitespace()).collect();

    let mut file = File::create(format!("./transcribed/{}.txt", title))
        .expect("Error: file could not be created");

    file.write_all(response.text.as_bytes())
        .expect("Error: Could not add text into file");

    println!("{:?}", response.text);

    Ok(())
}
