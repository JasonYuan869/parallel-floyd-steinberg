use crate::colors::get_color_tree;
use crate::convert::Converter;
use crate::convert_single_threaded::SingleThreadedConverter;

mod colors;
mod convert;
mod convert_mutex;
mod convert_single_threaded;
mod convert_channels;

const TEST_FILES: [&str; 3] = ["700x980.jpg", "1920x1000.png", "4128x6192.jpg"];

struct TestCase {
    name: &'static str,
    converter: Box<dyn Converter>,
}

fn main() -> anyhow::Result<()> {
    // Init the kd-tree
    get_color_tree();

    let mut test_cases: Vec<TestCase> = Vec::new();
    test_cases.push(TestCase {
        name: "single-threaded",
        converter: Box::new(SingleThreadedConverter::new()),
    });
    test_cases.push(TestCase {
        name: "mutex",
        converter: Box::new(convert_mutex::MutexConverter::new()),
    });
    test_cases.push(TestCase {
        name: "channels",
        converter: Box::new(convert_channels::ChannelConverter::new()),
    });

    for case in test_cases.iter() {
        println!("Running test case: {}", case.name);
        for file in TEST_FILES.iter() {
            print!("loading file: {}... ", file);
            let image = image::open(format!("./test_images/{file}"))?.to_rgb8();

            // Start time measurement
            let start = std::time::Instant::now();
            let result = case.converter.convert(image);

            // End time measurement
            let duration = start.elapsed();
            println!("time elapsed: {:?}", duration);

            // Save the converted image
            result.save(format!(
                "./test_images/converted_{}_{}.png", case.name, file
            ))?;
        }
    }



    Ok(())
}
