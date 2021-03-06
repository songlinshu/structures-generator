#![feature(core, hash, collections, io, fs)]
extern crate hyper;
extern crate toml;
extern crate regex;

#[macro_use]
mod macros;
mod utils;
mod cfg;
mod web;
mod prs;
mod pre;
mod tok;

#[cfg(not(test))]
fn main() {
	use std::fs::File;
	let config = cfg::load_config(&mut File::open("config.toml").unwrap()).unwrap();
	for page in web::fetch_contents(&config).unwrap().iter() {
		println!("[{}]", page.url);
		let code_blocks = web::find_code_blocks(&page.content);
		let cnt = code_blocks.len();
		match cnt {
			0 =>println!("no code blocks here, page size {}", page.content.len()),
			_ =>{
				println!("{} code block(s):", cnt);
				for block in code_blocks.iter() {
					match web::decode(block) {
						Ok(code) =>{
							let code = &pre::remove_single_line_comments(&code);
							match prs::compile(&mut code.chars()) {
								Ok(x) =>println!("{:?}", x),
								Err(e) =>println!("error: {}", e)
							}
						},
						Err(e) =>println!("error: {}", e)
					}
				}
			}
		}
	}
}
