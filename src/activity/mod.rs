use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::collections::HashMap;
use chrono::{Utc, NaiveDate, Datelike};
use colored::*;

#[derive(Serialize, Deserialize, Default)]
pub struct ActivityTracker {
    pub daily_counts: HashMap<String, ActivitySummary>,
    pub total_problems: u32,
    pub total_attempted: u32,
    pub total_solved: u32,
    pub streak_current: u32,
    pub streak_longest: u32,
}

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct ActivitySummary {
    pub attempted: u32,
    pub solved: u32,
}

#[derive(Debug)]
pub enum ActivityResult {
    NotAttempted,
    Attempted,
    Solved,
}

pub fn get_activity_file_path() -> Result<PathBuf> {
    let home_dir = std::env::var("HOME")
        .or_else(|_| std::env::var("USERPROFILE"))
        .map_err(|_| anyhow::anyhow!("Could not find home directory"))?;
    Ok(PathBuf::from(home_dir).join(".leetcli_activity.json"))
}

pub fn load_activity_tracker() -> Result<ActivityTracker> {
    let path = get_activity_file_path()?;
    if path.exists() {
        let content = fs::read_to_string(path)?;
        Ok(serde_json::from_str(&content)?)
    } else {
        Ok(ActivityTracker::default())
    }
}

pub fn save_activity_tracker(tracker: &ActivityTracker) -> Result<()> {
    let path = get_activity_file_path()?;
    let content = serde_json::to_string_pretty(tracker)?;
    fs::write(path, content)?;
    Ok(())
}

pub fn show_activity_graph() -> Result<()> {
    let tracker = load_activity_tracker()?;
    
    let now = Utc::now().date_naive();
    let current_year = now.year();
    let current_month = now.month();
    
    let first_day = NaiveDate::from_ymd_opt(current_year, current_month, 1)
        .ok_or_else(|| anyhow::anyhow!("Invalid first day of month"))?;
    
    let last_day = if current_month == 12 {
        NaiveDate::from_ymd_opt(current_year + 1, 1, 1)
            .unwrap()
            .pred_opt()
            .unwrap()
    } else {
        NaiveDate::from_ymd_opt(current_year, current_month + 1, 1)
            .unwrap()
            .pred_opt()
            .unwrap()
    };
    
    let month_name = match current_month {
        1 => "January", 2 => "February", 3 => "March", 4 => "April",
        5 => "May", 6 => "June", 7 => "July", 8 => "August",
        9 => "September", 10 => "October", 11 => "November", 12 => "December",
        _ => "Unknown",
    };
    
    println!("{} {}", month_name, current_year);
    println!();
    
    let mut calendar_start = first_day;
    while calendar_start.weekday().num_days_from_monday() != 0 {
        calendar_start = calendar_start.pred_opt().unwrap();
    }
    
    let mut calendar_end = last_day;
    while calendar_end.weekday().num_days_from_monday() != 6 {
        calendar_end = calendar_end.succ_opt().unwrap();
    }
    
    let total_days = (calendar_end - calendar_start).num_days() + 1;
    let total_weeks = (total_days / 7) as usize;
    
    print!("     ");
    for week in 0..total_weeks {
        let week_start = calendar_start + chrono::Duration::days(week as i64 * 7);
        if week_start.month() == current_month {
            print!("{:>3}", week_start.day());
        } else {
            print!("   ");
        }
    }
    println!();
    
    let week_days = ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];
    for (day_of_week, day_label) in week_days.iter().enumerate() {
        print!("{:<3}  ", day_label);
        
        for week in 0..total_weeks {
            let date = calendar_start + chrono::Duration::days(week as i64 * 7 + day_of_week as i64);
            
            if date.month() == current_month && date.year() == current_year {
                let date_str = date.format("%Y-%m-%d").to_string();
                let default_summary = ActivitySummary::default();
                let summary = tracker.daily_counts.get(&date_str).unwrap_or(&default_summary);
                
                let (_symbol, colored_symbol) = if summary.solved > 0 {
                    ("■", "■".green())
                } else if summary.attempted > 0 {
                    ("▣", "▣".yellow())
                } else {
                    ("□", "□".dimmed())
                };
                
                print!("{:>3}", colored_symbol);
            } else {
                print!("   ");
            }
        }
        println!();
    }
    
    println!();
    print!("Less ");
    print!("{}", "□".dimmed());
    print!(" ");
    print!("{}", "▣".yellow());
    print!(" ");
    print!("{}", "■".green());
    println!(" More");
    
    Ok(())
}

pub fn record_activity_completion(activity: &ActivityResult) -> Result<()> {
    let mut tracker = load_activity_tracker()?;
    let today = Utc::now().format("%Y-%m-%d").to_string();
    
    let summary = tracker.daily_counts.entry(today.clone()).or_insert(ActivitySummary::default());
    
    match activity {
        ActivityResult::Attempted => {
            summary.attempted += 1;
            tracker.total_attempted += 1;
        }
        ActivityResult::Solved => {
            summary.solved += 1;
            tracker.total_solved += 1;
            tracker.total_problems += 1;
        }
        ActivityResult::NotAttempted => {
        }
    }
    
    update_enhanced_streaks(&mut tracker)?;
    
    save_activity_tracker(&tracker)?;
    Ok(())
}

fn update_enhanced_streaks(tracker: &mut ActivityTracker) -> Result<()> {
    let mut dates: Vec<NaiveDate> = tracker.daily_counts.keys()
        .filter_map(|date_str| NaiveDate::parse_from_str(date_str, "%Y-%m-%d").ok())
        .filter(|date| {
            if let Some(summary) = tracker.daily_counts.get(&date.format("%Y-%m-%d").to_string()) {
                summary.attempted > 0 || summary.solved > 0
            } else {
                false
            }
        })
        .collect();
    dates.sort();
    
    if dates.is_empty() {
        return Ok(());
    }
    
    let today = Utc::now().date_naive();
    let mut current_streak = 0;
    let mut temp_streak = 0;
    let mut longest_streak = 0;
    
    let mut check_date = today;
    while let Some(summary) = tracker.daily_counts.get(&check_date.format("%Y-%m-%d").to_string()) {
        if summary.attempted > 0 || summary.solved > 0 {
            current_streak += 1;
            check_date = check_date.pred_opt().unwrap_or(check_date);
        } else {
            break;
        }
    }
    
    for window in dates.windows(2) {
        if let [prev, curr] = window {
            if (*curr - *prev).num_days() == 1 {
                temp_streak += 1;
            } else {
                longest_streak = longest_streak.max(temp_streak + 1);
                temp_streak = 0;
            }
        }
    }
    longest_streak = longest_streak.max(temp_streak + 1);
    
    tracker.streak_current = current_streak;
    tracker.streak_longest = longest_streak.max(tracker.streak_longest as i64) as u32;
    
    Ok(())
}

pub fn show_daily_progress() -> Result<()> {
    let tracker = load_activity_tracker()?;
    let today = Utc::now().format("%Y-%m-%d").to_string();
    let default_summary = ActivitySummary::default();
    let today_summary = tracker.daily_counts.get(&today).unwrap_or(&default_summary);
    
    println!("\n{}", "● Daily Progress".bright_cyan());
    println!("{}", "═══════════════".bright_cyan());
    
    if today_summary.solved > 0 {
        println!("✓ Problems solved today: {}", today_summary.solved.to_string().bright_green());
    }
    if today_summary.attempted > 0 {
        println!("# Problems attempted today: {}", today_summary.attempted.to_string().bright_yellow());
    }
    if today_summary.solved == 0 && today_summary.attempted == 0 {
        println!("◦ No activity today yet - time to start!");
    }
    
    println!("▓ Total solved: {}", tracker.total_solved.to_string().bright_green());
    println!("# Total attempted: {}", tracker.total_attempted.to_string().bright_yellow());
    println!("▲ Current streak: {} days", tracker.streak_current.to_string().bright_red());
    println!("★ Longest streak: {} days", tracker.streak_longest.to_string().bright_magenta());
    
    Ok(())
}