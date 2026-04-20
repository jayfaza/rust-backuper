use clap::Parser;

#[derive(Parser)]
#[command(about = "Archive your folders from src to dst!")]
pub struct Args {
    #[arg(short = 's', long = "src", help = "Source folder.")]
    pub src_directory: Option<String>,

    #[arg(
        short = 'd',
        long = "dst",
        help = "Destination folder. (default: ~/backup)"
    )]
    pub dst_directory: Option<String>,

    #[arg(
        long = "nvim",
        help = "The standard path to neovim config: '~/.config/nvim'"
    )]
    pub is_nvim: bool,
}
