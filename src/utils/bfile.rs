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
use byteorder::{ByteOrder, BigEndian, LittleEndian};

extern crate nalgebra_glm as glm;

pub struct BFile {
    fname: String,
    buffer: Vec<u8>,
    curpos: usize,
    big_end: bool
}


impl BFile {

    //"START" string (4 bytes)
    //Number Properties (4 byes)
    //Type Enum (4 bytes)
    //Size (4 bytes)
    //Name Enum (4 bytes)
    //Data (n bytes)

    pub fn new(fname: String, big_end: bool) -> Self {
        let mut f = File::open(&fname).expect("no file found");
        let metadata = fs::metadata(&fname).expect("unable to read metadata");
        let mut buffer = vec![0; metadata.len() as usize];
        f.read(&mut buffer).expect("buffer overflow");
        let mut bfile = BFile {
            fname: fname,
            buffer: buffer,
            curpos: 0,
            big_end: big_end
        };
        // let test_string: String = bfile.readString(5);
        // if test_string != "START" {
        //     panic!("corrupted file");
        // }
        return bfile;
    }

    pub fn readString(&mut self, size: usize) -> String {
        let (cow, encoding, has_errors) = UTF_8.decode(&self.buffer[self.curpos..self.curpos+size]);
        self.curpos += size;
        return match cow {
            Cow::Owned(s) => s,
            Cow::Borrowed(s) => s.to_string()
        };
    }

    pub fn autoReadString(&mut self) -> String {
        let size: usize = self.readu32() as usize;
        return self.readString(size);
    }

    pub fn readu32(&mut self) -> u32 {
        let res = if self.big_end {BigEndian::read_u32(&self.buffer[self.curpos..self.curpos+4])} else {LittleEndian::read_u32(&self.buffer[self.curpos..self.curpos+4])};
        self.curpos += 4;
        return res;
    }

    pub fn readi32(&mut self) -> i32 {
        let res = if self.big_end {BigEndian::read_i32(&self.buffer[self.curpos..self.curpos+4])} else {LittleEndian::read_i32(&self.buffer[self.curpos..self.curpos+4])};
        self.curpos += 4;
        return res;
    }

    pub fn readf32(&mut self) -> f32 {
        let res = if self.big_end {BigEndian::read_f32(&self.buffer[self.curpos..self.curpos+4])} else {LittleEndian::read_f32(&self.buffer[self.curpos..self.curpos+4])};
        self.curpos += 4;
        return res;
    }

    pub fn readbytes(&mut self, size: usize) -> Vec<u8> {
        let mut res = Vec::new();
        res.extend_from_slice(&self.buffer[self.curpos..self.curpos+size]);
        //res.reverse();
        self.curpos += size;
        return res;
    }

    pub fn readvec3(&mut self) -> glm::Vec3 {
        let ele1 = self.readf32();
        let ele2 = self.readf32();
        let ele3 = self.readf32();
        return glm::vec3(ele1 ,ele2, ele3);
    }

    pub fn isEnd(&mut self) -> bool {
        return self.curpos >= self.buffer.len();
    }
}