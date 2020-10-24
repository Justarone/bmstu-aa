use std::time::{ Duration, Instant };
use termion::{ style, color };
use chrono::{ DateTime, Utc };

#[derive(Clone)]
pub enum MetricsStatus {
    INIT,
    WAITING,
    WORKING,
}

#[derive(Clone)]
pub struct Metrics {
    status: MetricsStatus,
    cur_time: Instant,
    init_time: DateTime<Utc>,
    time_arr: Vec<(Duration, DateTime<Utc>, MetricsStatus)>,
}

impl Metrics {
    pub fn new() -> Self {
        Self {
            status: MetricsStatus::INIT,
            cur_time: Instant::now(),
            init_time: Utc::now(),
            time_arr: Vec::new(),
        }
    }

    fn process_checkpoint(&mut self) {
        self.time_arr.push((self.cur_time.elapsed(), Utc::now(), self.status.clone()));
        self.cur_time = Instant::now();
    }

    pub fn work(&mut self) {
        self.process_checkpoint();
        self.status = MetricsStatus::WORKING;
    }

    pub fn wait(&mut self) {
        self.process_checkpoint();
        self.status = MetricsStatus::WAITING;
    }

    fn print_interval(duration: &Duration, ts: &DateTime<Utc>, status: &MetricsStatus) {
        print!("{}{}", style::Bold, color::Fg(color::Black));
        match status {
            MetricsStatus::INIT => println!("{}INIT:    {:<10} ns {} {} {}", color::Bg(color::White),
                                            duration.as_nanos(), color::Bg(color::Blue), ts, style::Reset),
            MetricsStatus::WAITING => println!("{}WAITING: {:<10} ns {} {} {}", color::Bg(color::Red),
                                               duration.as_nanos(), color::Bg(color::Blue), ts, style::Reset),
            MetricsStatus::WORKING => println!("{}WORKING: {:<10} ns {} {} {}", color::Bg(color::Green),
                                               duration.as_nanos(), color::Bg(color::Blue), ts, style::Reset),
        }
    }

    pub fn print_all(&self) {
        println!("{}{}{}Metrics  created  in    {} {}", style::Bold, color::Bg(color::Blue),
        color::Fg(color::Black), self.init_time, style::Reset);
        for (duration, timestamp, status) in self.time_arr.iter() {
            Self::print_interval(duration, timestamp, status);
        }
    }
}
