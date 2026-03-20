//! Here you will add your macro_rule

//! Spawn a std::thread with loop for continuous update stuffs
#[macro_export]
macro_rules! loop_thread {
    ($seconds:expr, $body:block) => {
        std::thread::spawn(move || loop {
            $body;
            std::thread::sleep(std::time::Duration::from_secs($seconds));
        });
    };
    ($($body:tt)*) => {
        std::thread::spawn(move || loop {
            $($body)*;
            std::thread::sleep(std::time::Duration::from_millis(33)); // 30 FPS
        });
    };
}
