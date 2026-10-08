#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]

mod constants;
mod patterns;
mod impl_vasyexplore;
mod impl_vasyanalyse;

use crate::impl_vasyanalyse::vasy_analyse;
use std::{env};
use std::process::exit;

use clear_screen::clear;
use text_colorizer::{Colorize};

use crate::constants::{AUTHOR, NAME, VERSION, YEARS};
use crate::impl_vasyexplore::vasy_explore;

fn main()
{
	clear();
	println!("{} (version {} {YEARS} by {}) RUST {}",	NAME.bright_cyan().bold(),
																										Colorize::bold(VERSION),
																										Colorize::bold(AUTHOR),
																										rustc_version_runtime::version().to_string().bold());
	println!("Vu que Kate sous Windaube de merde c'est vraiment du stron...");

	let mut vecElements = Vec::<String>::new();
	let mut vecNomsPersos = Vec::<String>::new();

	let res_env = env::args().skip(1);

	if res_env.len() > 1 || res_env.len() == 0
	{
		println!("Usage: nameXtraktor <one file name or a complete directory>");
		exit(-1);
	}

	let mut RootPath :String = res_env.collect();
	if RootPath.eq(".")
	{
		RootPath = env::current_dir().unwrap().into_string().unwrap();
	}

	println!("Target is {}",RootPath);

	vasy_explore(RootPath, &mut vecElements);

	println!("Nombre de fichiers à traiter: {}",vecElements.len());

	for lesfichiersaouvrir in vecElements
	{
		println!("Ouverture de {}",lesfichiersaouvrir);
		vasy_analyse(lesfichiersaouvrir,&mut vecNomsPersos);
		println!("Nombre de personnages: {}",vecNomsPersos.len());
		for lesnoms in &vecNomsPersos
		{
			println!("\t Nom de perso: {}",lesnoms.bold().italic().truecolor(0xff,0xaa,0x00));
		}
		vecNomsPersos.clear();
	}
	exit(0);
} 
