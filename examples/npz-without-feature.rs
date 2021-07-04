use std::fs::File;
use std::io;

// Shows how to read and write NPZ archives without enabling the "npz" feature.

use npyz::{NpyReader, Builder, npz};

use zip::result::ZipResult;

fn main() -> io::Result<()> {
    write_a_file()?;
    read_a_file()?;
    Ok(())
}

fn write_a_file() -> ZipResult<()> {
    let file = io::BufWriter::new(File::create("examples/output/npz-without-feature.npz")?);

    // Create an NPZ with two arrays
    let mut zip = zip::ZipWriter::new(file);

    // Notice that we cannot use `begin_1d` because the Zip writer doesn't implement Seek.
    zip.start_file(npz::file_name_from_array_name("foo"), Default::default())?;
    let mut writer = Builder::new().default_dtype().begin_nd(&mut zip, &[6])?;
    for x in vec![1, 4, 7, 2, 3, 4] {
        writer.push(&x)?;
    }
    writer.finish()?;

    zip.start_file(npz::file_name_from_array_name("blah"), Default::default())?;
    let mut writer = Builder::new().default_dtype().begin_nd(&mut zip, &[2, 3])?;
    for x in vec![1.0, 4.0, 7.0, 2.0, 3.0, 4.0] {
        writer.push(&x)?;
    }
    writer.finish()?;

    zip.finish()?;
    Ok(())
}

fn read_a_file() -> ZipResult<()> {
    // read the file we just wrote in 'write_a_file'
    let file = io::BufReader::new(File::open("examples/output/npz-without-feature.npz")?);

    let mut zip = zip::ZipArchive::new(file)?;

    // here's how you would iterate over a file with unknown arrays
    for file_name in zip.file_names() {
        if let Some(array_name) = npz::array_name_from_file_name(&file_name) {
            println!("Found array: {}", array_name);
        }
    }
    println!();

    // In our case, though, we know the names
    let file = zip.by_name(&npz::file_name_from_array_name("foo"))?;
    let reader = NpyReader::<i32, _>::new(file)?;
    println!("ARRAY 'foo'", );
    println!("  shape: {:?}", reader.shape());
    println!("  dtype: {}", reader.dtype().descr());
    println!("  data: {:?}", reader.into_vec()?);

    let file = zip.by_name(&npz::file_name_from_array_name("blah"))?;
    let reader = NpyReader::<f64, _>::new(file)?;
    println!("ARRAY 'blah'", );
    println!("  shape: {:?}", reader.shape());
    println!("  dtype: {}", reader.dtype().descr());
    println!("  data: {:?}", reader.into_vec()?);

    Ok(())
}
