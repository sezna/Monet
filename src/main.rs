extern crate palette;
extern crate imageproc;
extern crate rsgenetic;
extern crate image;
extern crate clap;
extern crate rand;
mod stroke;
use stroke::Painting;
use rsgenetic::sim::par::Simulator;
use rsgenetic::sim::*;
use rsgenetic::sim::select::*;
use clap::{Arg, App};


fn main() {
    let args = App::new("Genetic Painter")
        .version("1.0")
        .author("Alexander Hansen <alex@alex-hansen.com>")
        .about("Paints a picture")
        .arg(
            Arg::with_name("population")
                .short("n")
                .long("population")
                .value_name("POPSIZE")
                .help("Sets the size of the initial population")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("strokes")
                .short("s")
                .long("strokes")
                .value_name("NUMSTROKES")
                .help("Sets the number of strokes per painting in the population")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("imagefile")
                .short("f")
                .long("file")
                .value_name("IMAGEFILE")
                .help("Selects the image file to use for painting")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("iterations")
                .short("i")
                .long("iterations")
                .value_name("NUMITER")
                .help(
                    "Sets the maximum number of iterations when running the \
                                   genetic algorithm",
                )
                .takes_value(true),
        )
        .arg(
            Arg::with_name("verbose")
                .short("v")
                .long("verbose")
                .help("Sets the verbosity level from 0 to 2")
                .takes_value(true),
        )
        .arg(Arg::with_name("random").short("r").long("random").help(
            "Sets the generation of the initial population to be random \
                                   instead of \"informed\"",
        ))
        .arg(
            Arg::with_name("selector")
                .short("e")
                .long("selector")
                .value_name("SELECTOR")
                .takes_value(true)
                .help(
                    "Picks the selector to use - stochastic, maximize, or tournament. \
			    Defaults to stochastic.",
                ),
        )
        .arg(
            Arg::with_name("strokewidth")
                .short("w")
                .long("strokewidth")
                .help("Sets the maximum width of strokes.")
                .value_name("WIDTH")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("maxcurve")
                .short("c")
                .long("maxcurve")
                .help(
                    "Sets the \
			     maximum curve value of a stroke",
                )
                .value_name("PIXELS")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("minstrokelength")
                .short("m")
                .long("minstrokelength")
                .help(
                    "Sets the minimum length of a \
			    stroke",
                )
                .takes_value(true)
                .value_name("LENGTH"),
        )
        .arg(
            Arg::with_name("maxstrokelength")
                .short("M")
                .long("maxstrokelength")
                .help(
                    "Sets the maximum length of a \
			    stroke",
                )
                .takes_value(true)
                .value_name("LENGTH"),
        )
        .get_matches();
    // Required args.
    let population: u32 = args.value_of("population").unwrap().parse().unwrap();
    let number_of_strokes: u32 = args.value_of("strokes").unwrap().parse().unwrap();
    let image_file = args.value_of("imagefile").unwrap();

    // Optional args.
    let iterations: u64 = args.value_of("iterations")
        .unwrap_or("100")
        .parse()
        .unwrap();
    let verbosity: u32 = args.value_of("verbose").unwrap_or("0").parse().unwrap();
    println!("verbosity set to {}", verbosity);
    let random_generation: bool = args.is_present("random");
    let width: u32 = args.value_of("strokewidth").unwrap_or("5").parse().unwrap();
    let selector = args.value_of("selector").unwrap_or("stochastic");
    let minlength: u32 = args.value_of("minstrokelength")
        .unwrap_or("5")
        .parse()
        .unwrap();
    let maxlength: u32 = args.value_of("maxstrokelength")
        .unwrap_or("150")
        .parse()
        .unwrap();
    let maxcurve: u32 = args.value_of("maxcurve").unwrap_or("5").parse().unwrap();

    println!(
        "{}",
        match verbosity {
            0 => String::new(),
            _ => {
                format!(
                    "Parameters: \n population: {} \nnumber of strokes: {}\n \
                               image_file: {}\n iterations: {}\n random \
                               generation is {}\n stroke width: {} \n\
			       min stroke length: {} max stroke length: {}\n \
			       max curve: {} ",
                    population,
                    number_of_strokes,
                    image_file,
                    iterations,
                    random_generation,
                    width,
                    minlength,
                    maxlength,
                    maxcurve
                )
            }

        }
    );

    println!("generating paintings");
    let mut population_paintings: Vec<Painting> = (0..population)
        .map(|_| if random_generation {
            Painting::random(
                image_file,
                number_of_strokes,
                width,
                minlength,
                maxlength,
                maxcurve,
            )
        } else {
            Painting::informed_random(image_file, number_of_strokes, width, minlength, maxlength)
        })
        .collect();
    if verbosity == 2 {
        // verbose debug prints
        println!("{} paintings added", population_paintings.len());
        println!("Now saving two sample images from the original population");
    }
    println!("saving two samples");
    population_paintings[0].render_painting("sample.png");
    population_paintings[1].render_painting("sample2.png");
    let s = Simulator::builder(&mut population_paintings).set_max_iters(iterations);

    // TODO figure out proper parameters and how tournament works
    println!("making simulator");
    let mut simulator = match selector {
        "stochastic" => {
            s.set_selector(Box::new(StochasticSelector::new(10)))
                .build()
        }
        "maximize" => s.set_selector(Box::new(MaximizeSelector::new(10))).build(),
        "tournament" => {
            s.set_selector(Box::new(TournamentSelector::new((population / 4) as usize, (population / 3 + (population / 3 ) % 2) as usize)))
                .build()
        }
        "parmaximize" => {
            s.set_selector(Box::new(ParMaximizeSelector::new(((population / 3) + population % 2) as usize)))
                .build()
        }
        _ => {
            println!("invalid selector provided, defaulting to stochastic");
            s.set_selector(Box::new(StochasticSelector::new(10)))
                .build()
        }
    };

    println!("running simulator");

    simulator.run();
    println!(
        "the most fit member is: {}",
        simulator.get().unwrap().fitness()
    );
    let most_fit = simulator.get().unwrap();

    // Save the output image.
    let filename = format!(
        "{}_i{}_s{}_p{}_r{}_mM{}-{}.png",
        selector,
        iterations,
        number_of_strokes,
        population,
        random_generation,
        minlength,
        maxlength
    );
    most_fit.render_and_save_image(filename);
}
