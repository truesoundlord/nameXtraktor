#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]

use std::{fs};
use std::process::{Command, ExitStatus};
use text_colorizer::Colorize;
use crate::patterns::{END_OF_SLICE, REGEX_CHARACTERTYPE, START_OF_SLICE};

pub fn analyse(path :String, lespersos :&mut  Vec<String>)
{
	// j'avais complètement oublié le fichier .duf est un fichier zippé :{

	let nompersonnage = String::new();
	let mut contenu;
	let mut position = None;
	let mut modifiedfilename = "".to_string();

	if cfg!(unix)
	{
		position = path.rfind('/');
	}

	if cfg!(windows)
	{
		position = path.rfind('\\');
	}

	let mut tmpdir;
	if position.is_some()
	{
		let (caca,_) = path.split_at(position.unwrap());
		tmpdir = caca.to_string();

		// println!("[DEBUG A.] tmpdir {}",tmpdir);

		if cfg!(unix)
		{
			tmpdir += "/tmp";
		}
		if cfg!(windows)
		{
			tmpdir += "\\tmp\\";																																					// rar needs trailing backslash...
		}
	}
	else
	{
		tmpdir = "tmp".to_string();
	}

	let _ = fs::create_dir(&tmpdir);
	let mut status :Result<ExitStatus,String> = Err("MERDE".to_string());
	
	if cfg!(unix)
	{
		status = Command::new("ark").arg("--batch").arg(&path).arg("-o").arg(tmpdir.clone()).status().map_err(|rustcaca| format!("Failed to use ark !! {}",rustcaca));
	}
	if cfg!(windows)
	{
		status = Command::new("unrar").arg("x").arg(&path).arg(tmpdir.clone()).status().map_err(|rustcaca| format!("Failed to use ark !! {}",rustcaca));
	}

	if status.unwrap().success()
	{
		println!("Fichier décompressé dans {}",tmpdir.bold().truecolor(0xb8,0xfb,0xff));

		if cfg!(unix)
		{
			position = path.rfind('/');
		}
		if cfg!(windows)
		{
			position = path.rfind('\\');
		}

		if position.is_none()
		{
			// le path ne contient pas un absolute path mais peut-être un nom de fichier simple
			modifiedfilename = path.to_string().replace(".duf", "");
			// println!("[B. DEBUG] {}",modifiedfilename);
		}
		else
		{
			let (_,tmp) = path.split_at(position.unwrap());
			if cfg!(unix)
			{
				modifiedfilename = tmp.to_string().replace("/", "").replace(".duf","");
			}
			if cfg!(windows)
			{
				modifiedfilename = tmp.to_string().replace("\\","").replace(".duf","");
			}
		}

		let mut pathcomplet = "".to_string();

		// println!("[C. DEBUG] {}",modifiedfilename);

		if cfg!(unix)
		{
			pathcomplet = format!("{}/{}",tmpdir,modifiedfilename);
		}
		if cfg!(windows)
		{
			pathcomplet = format!("{}\\{}",tmpdir,modifiedfilename);
		}

		let res_read = fs::read_to_string(pathcomplet);
		if res_read.is_err()
		{
			eprintln!("ERROR !! {}",res_read.unwrap_err());
			return;
		}

		contenu = res_read.unwrap();
		let res_regex = regex::Regex::new(REGEX_CHARACTERTYPE).expect("Oups REGEX caca !!");
		loop
		{
			let res_find = res_regex.find(&contenu);
			if res_find.is_some()
			{
				let mut position = res_find.unwrap().end();
				let (_,interet) = contenu.split_at(position);
				contenu = interet.to_string();
				let _res = contenu.find(START_OF_SLICE);
				if _res.is_some()
				{
					position = _res.unwrap();
					let (_,interet) = contenu.split_at(position+START_OF_SLICE.len());
					contenu = interet.to_string();
					let _res = contenu.find(END_OF_SLICE);
					if _res.is_some()
					{
						position = _res.unwrap();
						let (personnage,_) = contenu.split_at(position+END_OF_SLICE.len());
						lespersos.push(personnage.to_string().replace(END_OF_SLICE,""));
					}
				}
			}
			else
			{
				break;
			}
		} // endloop
		contenu.clear();
		let _ = fs::remove_dir_all(tmpdir);
	}
}
