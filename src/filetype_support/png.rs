/*
 * Copyright (C) 2025 Dustyn Gibb
 *
 * This program is free software; you can redistribute it and/or
 * modify it under the terms of the GNU General Public License
 * as published by the Free Software Foundation; either version 2
 * of the License, or (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program; if not, write to the Free Software
 * Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA  02110-1301, USA
 */

use crate::file_encoding_support::file_encoding_support::{
    FileEncoding, FileEncodingFunctionDerivation, FileEncodingMethod, FileEncodingSupport,
};
use crate::file_encoding_support::pixel::{embed_lsb_data_left_right, extract_lsb_data_left_right, Pixel};
use std::fs::File;
use std::io::{Read, Write};
use std::mem;
use std::process::exit;

const PNG_MAGIC : [u8;8] = [0x89,0x50,0x4E,0x47,0x0D,0x0A,0x1A,0x0A];
