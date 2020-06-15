use structopt::StructOpt;

#[derive(StructOpt)]
pub struct Args {
    /// The image format to save the clipboard's contents with
    pub format: String
}

pub fn args() -> Args {
    <_>::from_args()
}