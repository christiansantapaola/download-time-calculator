use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Opt {
    #[structopt(long = "kbps")]
    kbps: bool,
    #[structopt(long = "mbps")]
    mbps: bool,
    #[structopt(long = "gbps")]
    gbps: bool,
    #[structopt(long = "kb")]
    kb: bool,
    #[structopt(long = "mb")]
    mb: bool,
    #[structopt(long = "gb")]
    gb: bool,
    #[structopt(short = "v", long = "velocity")]
    speed: f64,
    #[structopt(short = "s", long = "size")]
    size: f64
}

enum SpeedUnit {
    Bps,
    Kbps,
    Mbps,
    Gbps
}

fn convert_speed_to_byte(value: f64, value_unit: SpeedUnit) -> f64 {
    match value_unit {
        SpeedUnit::Bps => value,
        SpeedUnit::Kbps => value * 1000.0,
        SpeedUnit::Mbps => value * 1000000.0,
        SpeedUnit::Gbps => value * 1000000000.0
    }
}

enum SizeUnit {
    B,
    Kb,
    Mb,
    Gb
}

fn convert_size_to_byte(value: f64, value_unit: SizeUnit) -> f64 {
    match value_unit {
        SizeUnit::B => value,
        SizeUnit::Kb => value * 1000.0,
        SizeUnit::Mb => value * 1000000.0,
        SizeUnit::Gb => value * 1000000000.0
    }
}

fn calc_time(size_byte : f64, speed_byte_per_second : f64) -> f64 {
    size_byte / speed_byte_per_second
}

fn main() {
    let opt = Opt::from_args();
    let size_unit = if opt.kb {
        SizeUnit::Kb
    } else if opt.mb {
        SizeUnit::Mb
    } else if opt.gb {
        SizeUnit::Gb
    } else {
        eprintln!("Byte");
        SizeUnit::B
    };
    let speed_unit = if opt.kbps {
        SpeedUnit::Kbps
    } else if opt.mbps {
        SpeedUnit::Mbps
    } else if opt.gbps {
        SpeedUnit::Gbps
    } else {
        eprintln!("Byte/s");
        SpeedUnit::Bps
    };
    let size = convert_size_to_byte(opt.size, size_unit);
    let speed = convert_speed_to_byte(opt.speed, speed_unit);
    let res = calc_time(size, speed).round() as i128;
    let hours = res / 3600;
    let remainder = res % 3600;
    let minutes = remainder / 60;
    let seconds = remainder % 60;
    if hours != 0 || minutes != 0 || seconds != 0 {
        println!("time estimated (HH:MM:SS): {}:{}:{}", hours, minutes, seconds);
    } else {
        println!("time estimated: less than 1 second");
    }
}
