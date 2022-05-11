use {
    rand::Rng,
    std::path::PathBuf,
    clipboard_win::{formats, get_clipboard}
};

mod args;

type Result<T = ()> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() {
    if let Err(e) = run() {
        eprint!("error: {e}")
    }
}

fn run() -> Result {
    let args = args::args();
    let image = clipboard_image()?;
    let temp_file = temp_file(&args.format);
    image.save(&temp_file)?;
    print!("{}", temp_file.display());

    Ok(())
}

fn clipboard_image() -> Result<image::DynamicImage> {
    let bitmap = get_clipboard(formats::Bitmap)
        .map_err(clipboard_error)?;
    Ok(image::load_from_memory(&bitmap)?)
}

fn clipboard_error(error: clipboard_win::SystemError) -> String {
    fn format_error(e: impl std::fmt::Display) -> String {
        format!("failed to get image from clipboard: {e}")
    }

    if error.is_zero() {
        format_error("content is not an image")
    } else {
        format_error(error)
    }
}

fn temp_file(format: &str) -> PathBuf {
    let mut temp_dir = std::env::temp_dir();
    let rand = rand::thread_rng().gen::<u64>();
    temp_dir.push(format!("clemp_{rand}"));
    temp_dir.set_extension(format);

    temp_dir
}
