use anyhow::Result;
use async_openai::{
    config::OpenAIConfig,
    types::{
        AudioInput, AudioResponseFormat, CreateTranscriptionRequest, CreateTranscriptionResponse,
    },
    Client,
};
use std::{fs::File, io::Write, path::PathBuf};
use youtube_dl::YoutubeDl;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = Client::new();

    let url: String = "https://www.youtube.com/watch?v=ulIkcUlZadw".to_string();
    let output_dir = "./audio".to_string();

    let title = youtube_downloader(url, output_dir).await?;

    let path = PathBuf::from(format!("./audio/{title}.m4a"));

    let transcibe_from_openai = openai_transcribe(
        client,
        path,
        "whisper-1".to_string(),
        None,
        None,
        None,
        Some("en".to_string()),
    )
    .await?;

    let title: String = title
        .to_lowercase()
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect();

    let text_file_output = format!("./transcribed/{title}.txt");

    save_output_into_folder(text_file_output, transcibe_from_openai).expect("Failed saving filed");

    Ok(())
}

async fn youtube_downloader(url: String, output_dir: String) -> Result<String> {
    let title = YoutubeDl::new(url)
        .extract_audio(true)
        .download(true)
        .format("m4a")
        .output_template("%(title)s.%(ext)s")
        .output_directory(output_dir)
        .run_async()
        .await?
        .into_single_video()
        .expect("Failed to get the title")
        .title;

    Ok(title)
}

async fn openai_transcribe(
    client: Client<OpenAIConfig>,
    path: PathBuf,
    model: String,
    prompt: Option<String>,
    response_format: Option<AudioResponseFormat>,
    temperature: Option<f32>,
    language: Option<String>,
) -> Result<CreateTranscriptionResponse> {
    let request = CreateTranscriptionRequest {
        file: AudioInput { path },
        model,
        prompt,
        response_format, //This loine does not really matter | CreateTranscriptionResponse returns text
        temperature,
        language,
    };

    let response = client
        .audio()
        .transcribe(request)
        .await?;

    Ok(response)
}

fn save_output_into_folder(path: String, response: CreateTranscriptionResponse) -> Result<()> {
    let mut file = File::create(path).expect("Error: file could not be created");

    println!("{}", response.text);

    file.write_all(response.text.as_bytes())
        .expect("Error: Could not add text into file");

    Ok(())
}
