use clap::{App, Arg};
use std::process;

fn main() {
    let matches = App::new("Paper Weight Calculator")
        .version("1.0")
        .author("Your Name")
        .about("Calculates the weight of printed data on A4 paper")
        .arg(
            Arg::with_name("DATA_SIZE")
                .help("The size of the data to calculate weight for (e.g., 500MB, 500M, 2GB, 2G)")
                .required(true)
                .index(1),
        )
        .get_matches();

    let data_size = matches.value_of("DATA_SIZE").unwrap();
    match calculate_weight(data_size) {
        Ok(weight) => println!("EN:The weight of the data is approximately {:.3} tonnes of A4 paper. \n\
                                CN:数据的重量约为 {:.3}吨A4纸。", weight,weight),
        Err(e) => {
            eprintln!("Error: {}", e);
            process::exit(1);
        }
    }
}

fn calculate_weight(data_size: &str) -> Result<f64, &'static str> {
    let bytes_per_page: f64 = 8000.0; // 每页大约8000字节
    let weight_per_page_g: f64 = 4.3659; // 每页大约4.3659克

    // 提取数字和单位
    // 提取数字和单位，允许单位有一个或两个字符
    let (size, unit) = if data_size.ends_with("MB") || data_size.ends_with("GB") {
        data_size.split_at(data_size.len() - 2)
    } else if data_size.ends_with("M") || data_size.ends_with("G") {
        data_size.split_at(data_size.len() - 1)
    } else {
        return Err("Invalid data size format. Please use M/MB for megabytes or G/GB for gigabytes.");
    };
    let size: f64 = size.parse().map_err(|_| "Invalid number format")?;


    // 根据单位转换为MB
    let size_mb = match unit.to_uppercase().as_str() {
        "M" | "MB" => size,
        "G" | "GB" => size * 1024.0,
        _ => return Err("Invalid unit. Use M/MB for megabytes or G/GB for gigabytes."),
    };

    // 计算总页数
    let total_pages = (size_mb * 1024.0 * 1024.0) / bytes_per_page;

    // 计算总重量（克）
    let total_weight_g = total_pages * weight_per_page_g;

    // 将克转换为吨
    let total_weight_tonnes = total_weight_g / 1_000_000.0;

    Ok(total_weight_tonnes)
}
