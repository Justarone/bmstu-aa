use std::sync::mpsc::{ Sender, Receiver, channel };
use std::thread;
use super::metrics::{ Metrics };

use super::task::{ RabinKarpTask, Task3 };
use super::additional_structs::{ RabinKarpTaskResult };

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
        let (metrics1_sender, metrics_reciever): (Sender<Metrics>, Receiver<Metrics>) = channel();

        let (metrics2_sender, metrics3_sender) = (metrics1_sender.clone(), metrics1_sender.clone());
        let mut metrics1 = Metrics::new();
        let (mut metrics2, mut metrics3) = (metrics1.clone(), metrics1.clone());

        thread::spawn(move || {
            metrics1.wait();
            for mut task in part1_receiver.iter() {
                metrics1.work();
                task.part1();
                metrics1.wait(); // maybe one line down?
                part1_sender.send(task).expect("Send task to part2");
            }
            metrics1_sender.send(metrics1).expect("Send metrics of layer to conveyor");
        });

        thread::spawn(move || {
            metrics2.wait();
            for mut task in part2_receiver.iter() {
                metrics2.work();
                task.part2();
                metrics2.wait(); // one line down?
                part2_sender.send(task).expect("Send task to part3");
            }
            metrics2_sender.send(metrics2).expect("Send metrics of layer to conveyor");
        });

        thread::spawn(move || {
            metrics3.wait();
            for mut task in part3_receiver.iter() {
                metrics3.work();
                task.part3();
                metrics3.wait();
                part3_sender.send(task).expect("Send task to output");
            }
            metrics3_sender.send(metrics3).expect("Send metrics of layer to conveyor");
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
