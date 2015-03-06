#![feature(core)]

extern crate portaudio;

use portaudio::pa;
use std::error::Error;

const SAMPLE_RATE: f64 = 44_100.0;
const FRAMES: usize = 1024;

fn init() -> pa::Stream<f32, f32>{
    match pa::initialize() {
        Ok(()) => println!("Successfully initialized PortAudio"),
        Err(err) => println!("An error occurred while initializing PortAudio: {}", err.description()),
    }

    println!("PortAudio host count : {}", pa::host::get_api_count() as isize);

    let default_host = pa::host::get_default_api();
    println!("PortAudio default host : {}", default_host as isize);

    match pa::host::get_api_info(default_host) {
        None => println!("Couldn't retrieve api info for the default host."),
        Some(info) => println!("PortAudio host name : {}", info.name),
    }


    let def_output = pa::device::get_default_output();
    let output_info = match pa::device::get_info(def_output) {
        Ok(info) => info,
        Err(err) => panic!("An error occurred while retrieving output info: {}", err.description()),
    };
    println!("Default output device name : {}", output_info.name);
    let out_params = pa::StreamParameters {
        device : def_output,
        channel_count : 2,
        sample_format : pa::SampleFormat::Float32,
        suggested_latency : output_info.default_high_output_latency
    };
    let mut stream : pa::Stream<f32, f32> = pa::Stream::new();
    match stream.open(None,
                      Some(&out_params),
                      SAMPLE_RATE,
                      FRAMES as u32,
                      pa::StreamFlags::ClipOff) {
        Ok(()) => println!("Successfully opened the stream."),
        Err(err) => println!("An error occurred while opening the stream: {}", err.description()),
    }

    match stream.start() {
        Ok(()) => println!("Successfully started the stream."),
        Err(err) => println!("An error occurred while starting the stream: {}", err.description()),
    }
    stream
}

fn drop(mut stream: pa::Stream<f32, f32>) {
    match stream.close() {
        Ok(()) => println!("Successfully closed the stream."),
        Err(err) => println!("An error occurred while closing the stream: {}", err.description()),
    }
    match pa::terminate() {
        Ok(()) => println!("Successfully terminated PortAudio."),
        Err(err) => println!("An error occurred while terminating PortAudio: {}", err.description()),
    }
}


fn buzz(stream: &pa::Stream<f32, f32>, buffer: &[f32]){
    'stream: loop {
        'waiting_for_stream: loop {
            match stream.get_stream_write_available() {
                Ok(None) => {
                    //println!("Not yet.");
                },
                Ok(Some(frames)) => {
                    println!("Write stream available with {} frames.", frames);
                    break 'waiting_for_stream
                },
                Err(err) => {
                    panic!("An error occurred while waiting for the stream: {}", err.description());
                },
            }
        }
        println!("Starting write");
        match stream.write(buffer, FRAMES as u32) {
            Ok(()) => {
                println!("Success");
            },
            Err(err) => {
                println!("Error when writing to the stream: {:?}", err);
                break 'stream;
            }
        }
    }
}



fn main() {
    let buffer : &mut [f32] = &mut [0.0; FRAMES][..];
    
    for val in buffer.iter_mut() {
        *val = 0.00;
    }
    println!("({:?}, {:?})", buffer, FRAMES);

    let stream = init();

    buzz(&stream, buffer);

    drop(stream);
}