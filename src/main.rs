use std::path::PathBuf;

const STEAM_FOLDERS: [&str; 6] = [
	r#"C:\Program Files (x86)\Steam"#,
	r#"C:\SteamLibrary"#,
	r#"D:\SteamLibrary"#,
	r#"E:\SteamLibrary"#,
	r#"F:\SteamLibrary"#,
	r#"G:\SteamLibrary"#,
];

enum Error {
	NoSteamLibraryFound, // Couldn't find an s&box game directory.
	GameNotFound(PathBuf), // Game wasn't found at steam directory PathBuf
	CitizenNotFound(PathBuf), // Couldn't find citizen model directory in sbox game path.

	IO(std::io::Error) // Couldn't delete folder, probably due to permissions.
}

fn find_sbox() -> Result<PathBuf, Error> {
	use std::path::Path;

	if let Ok(sbox_dir) = std::env::var("SBOX_DIR") {
		return Ok(sbox_dir.into());
	}

	for dir in STEAM_FOLDERS {
		let path = Path::new(dir);
		if path.is_dir() {
			let sbox_path = path.join("sbox");

			if !sbox_path.is_dir() {
				return Err( Error::GameNotFound( sbox_path ) );
			}
			return Ok(sbox_path);
		}
	};

	Err( Error::NoSteamLibraryFound )
}

fn handle_sbox(p: PathBuf) -> Result<(), Error> {
	use std::fs;
	let citizen_path = p
		.join("addons")
		.join("citizen")
		.join("models")
		.join("citizen");

	if !citizen_path.is_dir() {
		return Err( Error::CitizenNotFound(citizen_path) );
	}

	fs::remove_dir_all(citizen_path).map_err( Error::IO )?;
	Ok(())
}

fn uninstall() -> Result<(), Error> {
	let dir = find_sbox()?;
	handle_sbox(dir)?;

	Ok(())
}

fn main() {
	use Error::*;
	use colored::*;

	let err_text = "[ERROR]".red();
	match uninstall() {
		Err( NoSteamLibraryFound ) => println!( "{}: Couldn't find your SteamLibrary directory. Set the {} environment variable to bypass this.", err_text, "SBOX_DIR".bright_blue() ),
		Err( GameNotFound(sbox_dir) ) => println!( "{}: Steam game directory ['{}'] was found but s&box was not detected!", err_text, sbox_dir.parent().unwrap().display().to_string().italic().yellow() ),
		Err( CitizenNotFound(sbox_dir) ) => println!( "{}: Citizen folder was not found at in s&box directory ['{}']", err_text, sbox_dir.display().to_string().italic().yellow() ),
		Err( IO(ioerr) ) => println!( "{}: IO issue when deleting citizen folder. ['{}']", err_text, ioerr ),

		Ok(_) => println!( "{}: Uninstalled!", "[SUCCESS]".green() )
	}
}