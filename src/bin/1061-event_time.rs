use std::io::BufRead;

fn calculate_event_duration()
{
    const SECONDS_IN_DAY:u32 = 24 * 60 * 60;

    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines().map(|l| l.unwrap());

    let day_start: u32 = lines.next().unwrap()
        .split_whitespace().nth(1).unwrap()
        .parse().unwrap();

    let hour_start: Vec<u32> = lines.next().unwrap()
        .split(|c| c == ' ' || c == ':')
        .filter(|s| !s.is_empty())
        .map(|s| s.parse().unwrap())
        .collect();

    let day_end: u32 = lines.next().unwrap()
        .split_whitespace().nth(1).unwrap()
        .parse().unwrap();

    let hour_end: Vec<u32> = lines.next().unwrap()
        .split(|c| c == ' ' || c == ':')
        .filter(|s| !s.is_empty())
        .map(|s| s.parse().unwrap())
        .collect();

    let mut total_seconds:u32 = (day_end - day_start) * SECONDS_IN_DAY;
    let total_seconds_start:u32 = ((hour_start[0] * 60 * 60) + (hour_start[1] * 60) + hour_start[2]).into();
    let total_seconds_end:u32 = ((hour_end[0] * 60 * 60) + (hour_end[1] * 60) + hour_end[2]).into();

    if total_seconds == 0
    {
        total_seconds = total_seconds_end - total_seconds_start;
    }
    else
    {
        total_seconds = (day_end - day_start - 1) * SECONDS_IN_DAY;
        total_seconds += (SECONDS_IN_DAY - total_seconds_start) + total_seconds_end;
    }

    println!("{} dia(s)", total_seconds / SECONDS_IN_DAY);
    println!("{} hora(s)", total_seconds / 60 / 60 % 24);
    println!("{} minuto(s)", total_seconds / 60 % 60);
    println!("{} segundo(s)", total_seconds % 60);
}

fn main()
{
    calculate_event_duration();
}
