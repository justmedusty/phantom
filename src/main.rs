/*
    Copyright (C) 2025 Dustyn Gibb

This program is free software; you can redistribute it and/or
modify it under the terms of the GNU General Public License
as published by the Free Software Foundation; either version 2
of the License, or (at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program; if not, write to the Free Software
Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA  02110-1301, USA

 */
use std::env;
use std::process::CommandArgs;
use crate::arg_handling::arg_handling::arg_handling::parse_arguments;
use crate::file_encoding_support::file_encoding_support::{ImageSupport, Operation};

mod filetype_support;
mod file_encoding_support;
mod arg_handling;
mod mathematics_support;

fn main()  {
    let args : Vec<String> = env::args().collect();

    let mut image_support: ImageSupport = parse_arguments(args);
    
    match image_support.operation {
        Operation::Embed  => { image_support.encoding_support.embed_data(&mut image_support.data,image_support.encoding,image_support.encoding_method,image_support.file_encoding_function_derivation) },
        Operation::Extract=> { image_support.encoding_support.retrieve_data(image_support.data, image_support.encoding, image_support.encoding_method, image_support.file_encoding_function_derivation) }
    }


}
