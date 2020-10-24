use std::sync::mpsc::{ Sender, Receiver, channel };
use std::thread;
use std::time::{ Duration, Instant };
use termion::{ style, color };

use super::task::{ RabinKarpTask, Task3 };
use super::additional_structs::{ RabinKarpTaskResult };

#[derive(Clone)]
enum MetricsStatus {
    INIT,
    WAITING,
    WORKING,
}

#[derive(Clone)]
pub struct Metrics {
    status: MetricsStatus,
    cur_time: Instant,
    init_time: Duration,
    time_arr: Vec<(Duration, MetricsStatus)>,
}

impl Metrics {
    pub fn new() -> Self {
        Self {
            status: MetricsStatus::INIT,
            cur_time: Instant::now(),
            init_time: Duration::new(0, 0),
            time_arr: Vec::new(),
        }
    }

    fn process_checkpoint(&mut self) {
        self.time_arr.push((self.cur_time.elapsed(), self.status.clone()));
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

    pub fn init(&mut self) {
        self.status = MetricsStatus::INIT;
    }

    fn print_interval(duration: &Duration, status: &MetricsStatus) {
        print!("{}{}", style::Bold, color::Fg(color::Black));
        match status {
            MetricsStatus::INIT => println!("{}INIT: {} ns{}", color::Bg(color::White),
                                            duration.as_nanos(), style::Reset),
            MetricsStatus::WAITING => println!("{}WAITING: {} ns{}", color::Bg(color::Red),
                                               duration.as_nanos(), style::Reset),
            MetricsStatus::WORKING => println!("{}WORKING: {} ns{}", color::Bg(color::Green),
                                               duration.as_nanos(), style::Reset),
        }
    }

    pub fn print_all(&self) {
        for (duration, status) in self.time_arr.iter() {
            Self::print_interval(duration, status);
        }
    }
}

pub struct Conveyor3 {
    output: Receiver<RabinKarpTask>,
    metrics: Option<[Metrics; 3]>,
    metrics_channel: Option<Receiver<Metrics>>,
}

impl Conveyor3 {
    pub fn new(queue: Vec<RabinKarpTask>) -> Self {
        let (task_input, part1_receiver): (Sender<RabinKarpTask>, Receiver<RabinKarpTask>) = channel();
        let (part1_sender, part2_receiver): (Sender<RabinKarpTask>, Receiver<RabinKarpTask>) = channel();
        let (part2_sender, part3_receiver): (Sender<RabinKarpTask>, Receiver<RabinKarpTask>) = channel();
        let (part3_sender, task_output): (Sender<RabinKarpTask>, Receiver<RabinKarpTask>) = channel();
        let (metrics_sender, metrics_reciever): (Sender<Metrics>, Receiver<Metrics>) = channel();

        let metrics1_sender = metrics_sender.clone();
        thread::spawn(move || {
            let mut metrics = Metrics::new();
            metrics.wait();
            for mut task in part1_receiver.iter() {
                metrics.work();
                task.part1();
                metrics.wait(); // maybe one line down?
                part1_sender.send(task).expect("Send task to part2");
            }
            metrics1_sender.send(metrics).expect("Send metrics of layer to conveyor");
        });

        let metrics2_sender = metrics_sender.clone();
        thread::spawn(move || {
            let mut metrics = Metrics::new();
            metrics.wait();
            for mut task in part2_receiver.iter() {
                metrics.work();
                task.part2();
                metrics.wait(); // one line down?
                part2_sender.send(task).expect("Send task to part3");
            }
            metrics2_sender.send(metrics).expect("Send metrics of layer to conveyor");
        });

        let metrics3_sender = metrics_sender.clone();
        thread::spawn(move || {
            let mut metrics = Metrics::new();
            metrics.wait();
            for mut task in part3_receiver.iter() {
                metrics.work();
                task.part3();
                metrics.wait();
                part3_sender.send(task).expect("Send task to output");
            }
            metrics3_sender.send(metrics).expect("Send metrics of layer to conveyor");
        });

        for task in queue {
            task_input.send(task).expect("Send task to conveyor");
        }

        Self {
            output: task_output,
            metrics: None,
            metrics_channel: Some(metrics_reciever),
        }
    }

    pub fn recv(&self) -> Option<RabinKarpTaskResult> {
        self.output.recv().map(|res| res.result()).ok()
    }

    pub fn get_metrics(&mut self) -> Option<[Metrics; 3]> {
        if let Some(reciever) = self.metrics_channel.take() {
            self.metrics = Some([Metrics::new(), Metrics::new(), Metrics::new()]);
            if let Some(metrics_arr) = self.metrics.as_mut() {
                for (elem, ch_res) in metrics_arr.iter_mut().zip(reciever.iter()) {
                    *elem = ch_res;
                }
            }
        }

        self.metrics.clone()
    }
}
