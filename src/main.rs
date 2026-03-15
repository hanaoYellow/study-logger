use std::env;
use std::fs;
use serde::{Serialize, Deserialize};
use chrono::Local;

enum Command {
    Ad { subject: String, minutes: u32 },
    List,
    Summary,
}

#[derive(Serialize, Deserialize)]
struct Record {
    date : String,
    subject : String,
    minutes: u32,
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let command = parse_command(&args);

    match command {
        Command::Ad { subject, minutes } => add_record(&subject, minutes),
        Command::List => list_records(),
        Command::Summary => summary_records(),
    }
}

fn parse_command(args: &[String]) -> Command {
    match args[1].as_str() {
        "add" => Command::Ad {
            subject: args[2].clone(),
            minutes: args[3].parse().unwrap(),
        },
        "list" => Command::List,
        "summary" => Command::Summary,
        _ => panic!("不明なコマンドです"),
    }
}

fn add_record(subject: &str, minutes: u32) {
    let record = Record {
        date: Local::now().format("%Y-%m-%d").to_string(),
        subject: subject.to_string(),
        minutes,
    };

    // jsonファイルの読み込み
    let mut records: Vec<Record> = if std::path::Path::new("records.json").exists(){
        let json = fs::read_to_string("records.json").unwrap();
        serde_json::from_str(&json).unwrap()
    } else {
        Vec::new()
    };

    // 新しいrecordを追加
    records.push(record);

    let json = serde_json::to_string_pretty(&records).unwrap();
    fs::write("records.json", json).unwrap();

    println!("記録を追加しました: {} - {}分", subject, minutes);
}

fn list_records() {
    if std::path::Path::new("records.json").exists() {
        let json = fs::read_to_string("records.json").unwrap();
        let records: Vec<Record> = serde_json::from_str(&json).unwrap();
        
        std::println!("{:<11} {:<10} {:<10}", "date","subject","minutes");
        std::println!("{}", "-".repeat(32));

        for record in records {
            println!("{:<11} {:<10} {}分", record.date, record.subject, record.minutes);
        }

    } else {
        println!("記録がありません");
    }
}

fn summary_records() {
    if std::path::Path::new("records.json").exists() {
        let json = fs::read_to_string("records.json").unwrap();
        let records: Vec<Record> = serde_json::from_str(&json).unwrap();

        // 集計用のHashMap
        let mut summary: std::collections::HashMap<String, u32> = std::collections::HashMap::new();

        for record in records {
            *summary.entry(record.subject).or_insert(0) += record.minutes;
        }

        let mut summary_vec: Vec<(&String, &u32)> = summary.iter().collect();
        summary_vec.sort_by(|a, b| b.1.cmp(a.1));

        println!("科目別集計");
        println!("{}", "-".repeat(22));

        for (subject, total_minutes) in summary_vec {
            println!("{:<10} {}分", subject, total_minutes);
        }

    } else {
        println!("記録がありません");
    }
}