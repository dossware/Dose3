use sysinfo::{System, Networks, Disks, Components};

pub fn show_status() {
    let mut sys = System::new_all();
    sys.refresh_all();

    println!("\n🔧  System Information");
    println!("────────────────────────────────────────────");
    println!("📛 Hostname        : {}", System::host_name().unwrap_or_default());
    println!("🖥️ OS              : {}", System::name().unwrap_or_default());
    println!("🧬 Kernel          : {}", System::kernel_version().unwrap_or_default());
    println!("🧾 Version         : {}", System::os_version().unwrap_or_default());
    println!("💡 CPU Cores       : {}", sys.cpus().len());

    println!("\n📊 Memory Usage");
    println!("────────────────────────────────────────────");
println!("🧠 RAM Total       : {:.2} GB", sys.total_memory() as f64 / 1024.0 / 1024.0 / 1024.0);
println!("🧠 RAM Used        : {:.2} GB", sys.used_memory() as f64 / 1024.0 / 1024.0 / 1024.0);
println!("💾 Swap Total      : {:.2} GB", sys.total_swap() as f64 / 1024.0 / 1024.0 / 1024.0);
println!("💾 Swap Used       : {:.2} GB", sys.used_swap() as f64 / 1024.0 / 1024.0 / 1024.0);


    println!("\n📈 CPU/RAM Usage Chart");
    println!("────────────────────────────────────────────");
    show_cpu_ram_graph(&sys);

    println!("\n📦 Top Processes (by memory)");
    println!("────────────────────────────────────────────");
    let mut processes: Vec<_> = sys.processes().iter().collect();
    processes.sort_by_key(|(_, p)| -(p.memory() as i64));
    for (pid, proc) in processes.iter().take(5) {
        println!(
            "🧾 [{}] {:<20} - {:.2} MB",
            pid,
            proc.name().to_string_lossy(),
            proc.memory() as f64 / 1024.0 / 1024.0
        );
    }

    println!("\n💽 Disk Information");
    println!("────────────────────────────────────────────");
    let disks = Disks::new_with_refreshed_list();
    for disk in &disks {
        let fs = disk.file_system().to_string_lossy();
        println!(
            "📁 {:<10} | {} GB total | {} GB available | Type: {}",
            disk.mount_point().display(),
            disk.total_space() / 1_073_741_824,
            disk.available_space() / 1_073_741_824,
            fs
        );
    }

    println!("\n🌐 Network Interfaces");
    println!("────────────────────────────────────────────");
    let networks = Networks::new_with_refreshed_list();
    for (name, data) in &networks {
        println!(
            "🌍 {:<10} ↓ {:<10} B | ↑ {:<10} B",
            name,
            data.total_received(),
            data.total_transmitted()
        );
    }

    println!("\n🌡️ Components (Temperature Sensors)");
    println!("────────────────────────────────────────────");
    let mut components = Components::new();
    
    // Refresh multiple times to get more accurate readings
    components.refresh(false);
    std::thread::sleep(std::time::Duration::from_millis(500));
    components.refresh(false);
    
    let mut found_sensors = false;
    for component in &components {
        if let Some(temp) = component.temperature() {
            if temp > 0.0 { // Only show sensors with actual readings
                println!(
                    "🔥 {:<20} : {:.1}°C",
                    component.label(),
                    temp
                );
                found_sensors = true;
            }
        }
    }
    
    if !found_sensors {
        println!("⚠️  No temperature sensors detected or accessible");
        println!("   (This is common on some Windows systems)");
    }

    println!("\n✅ Status completed.\n");
}

fn show_cpu_ram_graph(sys: &System) {
    let avg_cpu: f32 = sys
        .cpus()
        .iter()
        .map(|cpu| cpu.cpu_usage())
        .sum::<f32>()
        / sys.cpus().len() as f32;

    let total = sys.total_memory() as f32;
    let used = sys.used_memory() as f32;
    let ram_percent = if total > 0.0 { (used / total) * 100.0 } else { 0.0 };

    print_bar("🧠 CPU Usage", avg_cpu);
    print_bar("📊 RAM Usage", ram_percent);
}

fn print_bar(label: &str, percent: f32) {
    let bar_width = 30;
    let filled = (percent / 100.0 * bar_width as f32).round() as usize;
    let bar = format!(
        "[{}{}] {:>5.1}%",
        "█".repeat(filled),
        " ".repeat(bar_width - filled),
        percent
    );
    println!("{:<15} {}", label, bar);
}
