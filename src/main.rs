use std::collections::HashSet;
use std::io;
use std::process;
use std::error::Error;
use csv::Writer;
use indicatif::{ProgressBar, ProgressStyle};

fn main() {
    let mut values :Vec<Vec<String>> =vec![];

    if let Err(err) = read_csv(&mut values) {
        println!("error running example: {}", err);
        process::exit(1);
    }


    //get atributes and classes 
    let mut classes: Vec<&str> = vec![];
    let mut attributes :Vec<Vec<String>> =vec![];

    for j in values.iter() {
        classes.push(j[values[0].len()-1].as_str());
        attributes.push(j[..values[0].len()-1].to_vec());
    }
    

    let pb = ProgressBar::new(classes.len() as u64);
    pb.set_style(ProgressStyle::with_template("[{elapsed_precise}/{eta}] {bar:60.cyan/blue} {pos:>7}/{len:7} {msg}")
    .unwrap()
    .progress_chars(">--"));
    let mut dv :Vec<Vec<u8>> =vec![];

    eprintln!("Creating Discernibility Matrix");
    //BDM
    //discenibility vectors
    for (i,j) in classes.iter().enumerate() {
        pb.inc(1);
        for (l,k) in classes[i+1..].iter().enumerate(){
            if k != j {
                dv.push(compare(&attributes[i], &attributes[i+l+1]))
            }
        }
    }
    pb.finish();
    eprintln!("Finished Dicernibility Matrix");
    let mut bm: Vec<Vec<u8>> = dv.clone();
    let mut rows:Vec<usize> = vec![];

    let pb = ProgressBar::new(dv.len() as u64+ rows.len() as u64);
    pb.set_style(ProgressStyle::with_template("[{elapsed_precise}/{eta}] {bar:60.cyan/blue} {pos:>7}/{len:7} {msg}").unwrap()
    .progress_chars(">--"));
    
    eprintln!("Checking for zeros");
    for (i,j) in dv.iter().enumerate(){
        pb.inc(1);
        if is_zero(j) {
            rows.push(i);
            continue;
       }
    }

    for j in rows.into_iter().rev() {
        pb.inc(1);
        bm.remove(j);
    }
    pb.finish();
    eprintln!("Finished checking for zeros");

    let pb = ProgressBar::new(bm.len() as u64);
    pb.set_style(ProgressStyle::with_template("[{elapsed_precise}/{eta}] {bar:60.cyan/blue} {pos:>7}/{len:7} {msg}").unwrap()
    .progress_chars(">--"));
    
    eprintln!("Creating basic matrix");
    let mut rows:Vec<u8> = vec![];

    'loop1:for (i,j) in bm.iter().enumerate(){
        pb.inc(1);
        for (k,l) in bm[i+1..].iter().enumerate(){
            if less(&j, &l){
                rows.push((k+i+1) as u8);
            }else if less(&l,&j){
                rows.push(i as u8);
                continue 'loop1
            }
        }
    }
    pb.finish();
    let mut rows = filter_uniq(rows);
    rows.sort();
    for j in rows.into_iter().rev() {
        bm.remove(j as usize);
    }

    eprintln!("Finished basic matrix");

    to_csv(&bm).expect("Error");
}

fn read_csv(values: &mut Vec<Vec<String>>) -> Result<&Vec<Vec<String>>,Box<dyn Error>> {
    let mut rdr = csv::Reader::from_reader(io::stdin()); //reading whole
    for result in rdr.records() {
        let mut row:Vec<String> = vec![];
        for value in result?.into_iter(){
            row.push(value.to_string());
        }
        values.push(row);
    }
    Ok(values)
}

fn compare(row1:&Vec<String>,row2:&Vec<String>) -> Vec<u8>{
    let mut x:Vec<u8> = vec![];
    for (i,j) in row1.into_iter().enumerate(){
        if j == &row2[i] {x.push(0)} else {x.push(1)};
    }
    x
}

fn metric(row1:&Vec<String>,row2:&Vec<String>) -> Vec<u8>{
    let mut x:Vec<u8> = vec![];
    for (i,j) in row1.into_iter().enumerate(){
        if j == &row2[i] {x.push(0)} else {x.push(1)};
    }
    x
}
//much faster
fn is_zero(buf: &Vec<u8>) -> bool {
    let (prefix, aligned, suffix) = unsafe { buf.align_to::<u128>() };

    prefix.iter().all(|&x| x == 0)
        && suffix.iter().all(|&x| x == 0)
        && aligned.iter().all(|&x| x == 0)
}

fn less(row1:&Vec<u8>,row2:&Vec<u8>) -> bool {
    let mut _x:u32 = 0;
    for (i,j) in row1.iter().enumerate(){
        if j > &row2[i]{
            _x +=1;
            break
        }else{
            continue
        }
    }
    if _x!=0 {false} else {true}
}

fn filter_uniq(vec: Vec<u8>) -> Vec<u8> {
    vec.into_iter()
        .collect::<HashSet<u8>>()
        .into_iter()
        .collect()
}

fn to_csv(vector:&Vec<Vec<u8>>) -> Result<(), Box<dyn Error>> {

    eprintln!("Creating CSV");
    let pb = ProgressBar::new(vector.len() as u64);
    pb.set_style(ProgressStyle::with_template("[{elapsed_precise}/{eta}] {bar:60.cyan/blue} {pos:>7}/{len:7} {msg}").unwrap()
    .progress_chars(">--"));
    let mut wtr = Writer::from_path("../../basicMatrix.csv")?;
    for k in vector.into_iter(){
        pb.inc(1);
        wtr.write_record(k.iter().map(|e| e.to_string()))?;
    }
    pb.finish();
    eprintln!("Finished CSV");
    wtr.flush()?;
    Ok(())
}
