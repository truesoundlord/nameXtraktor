#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]

use std::env;
use std::fs::{read_dir};

pub fn vasy_explore(root :String, vecteur :&mut Vec<String>)
{
	let res_open = read_dir(root.clone());

	if res_open.is_err()
	{
		// Pas un répertoire donc il s'agit d'un fichier...
		if root.ends_with(".duf")
		{
			let current = env::current_dir().unwrap();
			let filename = format!("{}/{}",current.to_string_lossy(),root);
			vecteur.push(filename);
		}
		return;
	}

	for uneentree in res_open.unwrap()
	{
		if uneentree.is_ok()
		{
			let binding = uneentree.unwrap();
			if binding.file_type().unwrap().is_dir()
			{
				vasy_explore(binding.path().into_string().unwrap(), vecteur);
			}
			if binding.file_type().unwrap().is_file()
			{
				let filename = binding.path();
				if filename.clone().into_string().unwrap().ends_with(".duf")
				{
					vecteur.push(filename.into_string().unwrap());
				}
			}
		}
	}
}