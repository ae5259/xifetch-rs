use std::process::Command;

mod color;
mod config;

fn main() {
    config::config();
    let fetch = color::colorize(print_fetch());

    println!("{}", fetch);
}

fn execute_command(command: &str, arg: Option<&str>) -> String {
    let mut command = &mut Command::new(command);

    if arg.is_some() {
        command = command.arg(arg.unwrap());
    }

    match command.output() {
        Ok(result) => {
            if result.status.success() {
                return String::from_utf8_lossy(&result.stdout).trim().to_string();
            } else {
                return format!(
                    "Command failed (status: {:?}): {}",
                    result.status,
                    String::from_utf8_lossy(&result.stderr)
                );
            }
        }

        Err(err) => format!("Failed to execute command: {}", err),
    }
}

fn get_os_name() -> String {
    execute_command("uname", Some("-o"))
}

fn get_host_name() -> String {
    execute_command("hostname", None)
}

fn get_kernel_version() -> String {
    execute_command("uname", Some("-r"))
}

fn get_uptime() -> String {
    let output = execute_command("uptime", None);
    let Some(up_index) = output.find("up") else {
        return String::from("XZ BRO");
    };

    let time_str = &output[up_index + 3..];
    let Some(comma_index) = time_str.find(',') else {
        return String::from("XZ BRO");
    };

    let extracted_time = &time_str[..comma_index];
    let trimmed_time = extracted_time.trim();

    return trimmed_time.to_string();
}

fn get_cpu_info() -> Option<String> {
    let output = execute_command("lscpu", None);

    let model_name_line = output.lines().find(|line| line.contains("Model name"))?;
    let model_name = model_name_line.split(':').nth(1)?.trim();
    let cpu_cores_line = output.lines().find(|line| line.contains("CPU(s)"))?;
    let cpu_cores = cpu_cores_line.split(':').nth(1)?.trim();
    Some(format!("{} ({} cores)", model_name, cpu_cores))
}

fn get_memory_info() -> Option<String> {
    let output = execute_command("free", Some("-h"));

    let lines: Vec<&str> = output.lines().collect();
    let mem_line = lines.iter().find(|line| line.contains("Mem:"))?;
    let mem_parts: Vec<&str> = mem_line.split_whitespace().collect();
    let total_mem = mem_parts[1];
    let used_mem = mem_parts[2];
    Some(format!("{}  / {} ", used_mem, total_mem))
}

fn print_fetch() -> String {
    let os_name = get_os_name();
    let host_name = get_host_name();
    let kernel_version = get_kernel_version();
    let uptime = get_uptime();
    let cpu = get_cpu_info();
    let memory = get_memory_info();

    let mut result: String = String::new();

    result.push_str(format!("OS: {}\n", os_name).as_str());
    result.push_str(format!("Host: {}\n", host_name).as_str());
    result.push_str(format!("Kernel: {}\n", kernel_version).as_str());
    result.push_str(format!("Uptime: {}\n", uptime).as_str());
    result.push_str(format!("CPU: {}\n", cpu.unwrap()).as_str());
    result.push_str(format!("Memory: {}\n", memory.unwrap()).as_str());

    result
}
