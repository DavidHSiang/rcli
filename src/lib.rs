mod cli;
mod process;
mod utils;

pub use cli::{
    Base64DecodeOpts, Base64EncodeOpts, Base64SubCommand, CsvOpts, GenPassOpt, HttpServeOpts,
    HttpSubCommand, KeyGenerateOpts, Opts, SubCommand, TextSignOpts, TextSubCommand,
    TextVerifyOpts,
};
use enum_dispatch::enum_dispatch;
pub use process::{
    process_csv, process_decode, process_encode, process_genpass, process_http_serve,
    process_text_key_generate, process_text_sign, process_text_verify,
};
pub use utils::{get_content, get_reader};

#[allow(async_fn_in_trait)]
#[enum_dispatch]
pub trait CmdExector {
    async fn execute(self) -> anyhow::Result<()>;
}
