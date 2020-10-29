use std::sync::mpsc::{ Sender, Receiver, channel };
use std::thread;

use super::task::{ RabinKarpTask, Task3 };
use super::additional_structs::{ RabinKarpTaskResult };

pub struct Conveyor3 {
    output: Receiver<RabinKarpTask>,
}

impl Conveyor3 {
    pub fn new(queue: Vec<RabinKarpTask>) -> Self {
        let (task_input, part1_receiver): (Sender<RabinKarpTask>, Receiver<RabinKarpTask>) = channel();
        let (part1_sender, part2_receiver): (Sender<RabinKarpTask>, Receiver<RabinKarpTask>) = channel();
        let (part2_sender, part3_receiver): (Sender<RabinKarpTask>, Receiver<RabinKarpTask>) = channel();
        let (part3_sender, task_output): (Sender<RabinKarpTask>, Receiver<RabinKarpTask>) = channel();

        thread::spawn(move || {
            for mut task in part1_receiver.iter() {
                task.part1();
                part1_sender.send(task).expect("Send task to part2");
            }
        });

        thread::spawn(move || {
            for mut task in part2_receiver.iter() {
                task.part2();
                part2_sender.send(task).expect("Send task to part3");
            }
        });

        thread::spawn(move || {
            for mut task in part3_receiver.iter() {
                task.part3();
                part3_sender.send(task).expect("Send task to output");
            }
        });

        for task in queue {
            task_input.send(task).expect("Send task to conveyor");
        }

        Self {
            output: task_output,
        }
    }

    pub fn recv(&self) -> Option<RabinKarpTaskResult> {
        self.output.recv().map(|res| res.result()).ok()
    }
}
