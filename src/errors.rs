use error_chain::error_chain;

error_chain!{
	foreign_links {
		Io(std::io::Error);
		SerdeJson(serde_json::Error);
		Clap(clap::Error);
	}
}