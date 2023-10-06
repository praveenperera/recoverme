use crossbeam::channel::{Receiver, Sender};
use indicatif::ProgressStyle;

pub struct ProgressBar {
    pub receiver: Receiver<()>,
    pub total: u128,
    pub sender: Sender<()>,
}

impl ProgressBar {
    pub fn new(total: u128) -> Self {
        let (sender, receiver) = crossbeam::channel::bounded(1000);

        Self {
            receiver,
            total,
            sender,
        }
    }

    pub fn listen(self) {
        std::thread::spawn(move || {
            let progress_bar = indicatif::ProgressBar::new(self.total as u64);

            progress_bar.set_style(
                ProgressStyle::with_template(
                    "[{elapsed_precise}] [ETA: {eta}] {wide_bar}  {human_pos} / {human_len}  ({per_sec}) ",
                )
                .unwrap()
            );

            for (current, _) in self.receiver.iter().enumerate() {
                if current % 1000 == 0 {
                    progress_bar.inc(1000);
                }
            }

            progress_bar.finish();
        });
    }
}
