#![allow(clippy::from_over_into)]
mod githook;
mod json;
mod rummit;

use githook::githook_retriever::GithookRetriever;
use json::json_loader::JsonLoader;
use rummit::app::App;
use rummit::commit_retriever::CommitRetriever;
use rummit::options_loader::OptionsLoader;
use std::process::ExitCode;

fn main() -> ExitCode {
	// app configuration
	let commit_retriever: Box<dyn CommitRetriever> = Box::new(GithookRetriever::new());
	let options_loader: Box<dyn OptionsLoader> = Box::new(JsonLoader::new());
	let app = App::new(commit_retriever, options_loader);

	app.run()
}
