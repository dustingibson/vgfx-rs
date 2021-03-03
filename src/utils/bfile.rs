use gl;
use gl::types::*;
use std::fs;
use std::ffi::CString;
use std::os::raw::c_char;
use std::ptr::null;
use std::ptr::null_mut;
use std::mem;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use encoding_rs::Decoder;
use encoding_rs::UTF_8;
use std::borrow::Cow;
use byteorder::{ByteOrder, BigEndian};

extern crate nalgebra_glm as glm;

pub struct BFile {
    fname: String,
    buffer: Vec<u8>,
    curpos: usize
}


impl BFile {

    //"START" string (4 bytes)
    //Number Properties (4 byes)
    //Type Enum (4 bytes)
    //Size (4 bytes)
    //Name Enum (4 bytes)
    //Data (n bytes)

    pub fn new(fname: String) -> Self {
        let mut f = File::open(&fname).expect("no file found");
        let metadata = fs::metadata(&fname).expect("unable to read metadata");
        let mut buffer = vec![0; metadata.len() as usize];
        f.read(&mut buffer).expect("buffer overflow");
        // 53 54 41 52
        return BFile {
            fname: fname,
            buffer: buffer,
            curpos: 0
        }
    }

    pub fn readString(&mut self, size: usize) -> String {
        let (cow, encoding, has_errors) = UTF_8.decode(&self.buffer[self.curpos..size]);
        self.curpos += size;
        return match cow {
            Cow::Owned(s) => s,
            Cow::Borrowed(s) => s.to_string()
        };
    }

    pub fn readu32(&mut self) -> u32 {
        let res = BigEndian::read_u32(&self.buffer[self.curpos..self.curpos+4]);
        self.curpos += 4;
        return res;
    }

    pub fn readi32(&mut self) -> i32 {
        let res = BigEndian::read_i32(&self.buffer[self.curpos..self.curpos+4]);
        self.curpos += 4;
        return res;
    }

    pub fn readuf32(&mut self) -> f32 {
        let res = BigEndian::read_f32(&self.buffer[self.curpos..self.curpos+4]);
        self.curpos += 4;
        return res;
    }
}