extern crate crossbeam;

use std::thread;
use crossbeam_channel::unbounded;
use rayon::prelude::{IntoParallelIterator, ParallelIterator};

fn slice_to_number(data_segment: &&str) -> u32 {
    data_segment
        .chars()
        .filter_map(|c| c.to_digit(10))
        .sum::<u32>()
}

pub fn test_crossbeam_impl(data_segments: &[&str]) {
    let (snd, rcv) = unbounded();
    let mut _result = 0;
    let mut nsegments = 0;

    crossbeam::scope(|spanner|{
        for data_segment in data_segments.iter() {
            nsegments += 1;

            spanner.spawn(|_| {
                let result = slice_to_number(data_segment);
                snd.send(result).unwrap();
            });
        }
    }).unwrap();

    for _ in 0..nsegments {
        _result += rcv.recv().unwrap_or(0);
    }
}

pub fn test_par_iter_impl(data_segments: &[&str]) {
    let _result = data_segments
        .into_par_iter()
        .map(|segment| -> u32 { 
            slice_to_number(segment) 
        })
        .sum::<u32>();
}

pub fn test_buildin_thread_impl(data_segments: &[&str]) {
    let mut thread_pool = vec![];

    for data_segment in data_segments.iter() {
        let data_segment_str = data_segment.to_string();

        thread_pool.push(thread::spawn(move || -> u32 {
            slice_to_number(&data_segment_str.as_ref())
        }));  
    }
        
    let _final_result = thread_pool
        .into_iter()
        .map(|t| t.join().unwrap())
        .sum::<u32>();
}

