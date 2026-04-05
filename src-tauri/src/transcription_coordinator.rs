use crate::actions::ACTION_MAP;
use crate::managers::audio::AudioRecordingManager;
use log::{debug, error, warn};
use std::sync::mpsc::{self, Sender};
use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};
use tauri::{AppHandle, Manager};

const DEBOUNCE: Duration = Duration::from_millis(30);
const LONG_PRESS_THRESHOLD: Duration = Duration::from_millis(300); // 长按阈值：300ms

/// Commands processed sequentially by the coordinator thread.
enum Command {
    Input {
        binding_id: String,
        hotkey_string: String,
        is_pressed: bool,
        push_to_talk: bool,
        has_modifiers: bool, // 是否有其他修饰键同时按下
    },
    Cancel {
        recording_was_active: bool,
    },
    ProcessingFinished,
    LongPressDetected { binding_id: String }, // 长按检测到的命令
}

/// Pipeline lifecycle, owned exclusively by the coordinator thread.
enum Stage {
    Idle,
    Recording(String), // binding_id
    Processing,
}

/// Serialises all transcription lifecycle events through a single thread
/// to eliminate race conditions between keyboard shortcuts, signals, and
/// the async transcribe-paste pipeline.
pub struct TranscriptionCoordinator {
    tx: Sender<Command>,
}

pub fn is_transcribe_binding(id: &str) -> bool {
    id == "transcribe" || id == "transcribe_with_post_process"
}

impl TranscriptionCoordinator {
    pub fn new(app: AppHandle) -> Self {
        let (tx, rx) = mpsc::channel();
        let tx_clone = tx.clone();

        thread::spawn(move || {
            let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                let mut stage = Stage::Idle;
                let mut last_press: Option<Instant> = None;
                let mut _press_start_time: Option<Instant> = None;
                let mut long_press_timer: Option<thread::JoinHandle<()>> = None;
                let mut pending_binding_id: Option<String> = None;

                while let Ok(cmd) = rx.recv() {
                    match cmd {
                        Command::Input {
                            binding_id,
                            hotkey_string,
                            is_pressed,
                            push_to_talk,
                            has_modifiers,
                        } => {
                            // Debounce rapid-fire press events (key repeat / double-tap).
                            if is_pressed {
                                let now = Instant::now();
                                if last_press.map_or(false, |t| now.duration_since(t) < DEBOUNCE) {
                                    debug!("Debounced press for '{binding_id}'");
                                    continue;
                                }
                                last_press = Some(now);
                            }

                            // 如果有其他修饰键同时按下，取消当前录音（如果正在进行）
                            if is_pressed && has_modifiers {
                                if matches!(&stage, Stage::Recording(_)) {
                                    debug!("Modifiers detected with '{binding_id}', cancelling recording");
                                    cancel_recording(&app, &mut stage);
                                }
                                continue;
                            }

                            if push_to_talk {
                                if is_pressed && matches!(stage, Stage::Idle) {
                                    // 记录按下时间
                                    _press_start_time = Some(Instant::now());
                                    pending_binding_id = Some(binding_id.clone());
                                    
                                    // 启动长按检测定时器
                                    let tx_timer = tx_clone.clone();
                                    let binding_id_timer = binding_id.clone();
                                    long_press_timer = Some(thread::spawn(move || {
                                        thread::sleep(LONG_PRESS_THRESHOLD);
                                        let _ = tx_timer.send(Command::LongPressDetected {
                                            binding_id: binding_id_timer,
                                        });
                                    }));
                                    
                                    debug!("Push-to-talk: waiting for long press on '{binding_id}'");
                                } else if !is_pressed {
                                    // 释放按键
                                    
                                    // 取消长按定时器
                                    if let Some(timer) = long_press_timer.take() {
                                        drop(timer);
                                    }
                                    
                                    if matches!(&stage, Stage::Recording(id) if id == &binding_id) {
                                        // 如果正在录音，停止录音
                                        stop(&app, &mut stage, &binding_id, &hotkey_string);
                                    }
                                    // 短按不做任何操作
                                    
                                    _press_start_time = None;
                                    pending_binding_id = None;
                                }
                            } else if is_pressed {
                                match &stage {
                                    Stage::Idle => {
                                        start(&app, &mut stage, &binding_id, &hotkey_string);
                                    }
                                    Stage::Recording(id) if id == &binding_id => {
                                        stop(&app, &mut stage, &binding_id, &hotkey_string);
                                    }
                                    _ => {
                                        debug!("Ignoring press for '{binding_id}': pipeline busy")
                                    }
                                }
                            }
                        }
                        Command::LongPressDetected { binding_id } => {
                            // 长按检测触发，开始录音
                            if matches!(stage, Stage::Idle) && pending_binding_id.as_ref() == Some(&binding_id) {
                                debug!("Long press detected for '{binding_id}', starting recording");
                                start(&app, &mut stage, &binding_id, &binding_id);
                            }
                            long_press_timer = None;
                        }
                        Command::Cancel {
                            recording_was_active,
                        } => {
                            // Don't reset during processing — wait for the pipeline to finish.
                            if !matches!(stage, Stage::Processing)
                                && (recording_was_active || matches!(stage, Stage::Recording(_)))
                            {
                                stage = Stage::Idle;
                            }
                        }
                        Command::ProcessingFinished => {
                            stage = Stage::Idle;
                        }
                    }
                }
                debug!("Transcription coordinator exited");
            }));
            if let Err(e) = result {
                error!("Transcription coordinator panicked: {e:?}");
            }
        });

        Self { tx }
    }

    /// Send a keyboard/signal input event for a transcribe binding.
    /// For signal-based toggles, use `is_pressed: true` and `push_to_talk: false`.
    pub fn send_input(
        &self,
        binding_id: &str,
        hotkey_string: &str,
        is_pressed: bool,
        push_to_talk: bool,
    ) {
        // 检测是否有其他修饰键（用于检测组合键）
        let has_modifiers = hotkey_string.contains('+') && !hotkey_string.starts_with("command") && !hotkey_string.starts_with("cmd");
        
        if self
            .tx
            .send(Command::Input {
                binding_id: binding_id.to_string(),
                hotkey_string: hotkey_string.to_string(),
                is_pressed,
                push_to_talk,
                has_modifiers,
            })
            .is_err()
        {
            warn!("Transcription coordinator channel closed");
        }
    }

    pub fn notify_cancel(&self, recording_was_active: bool) {
        if self
            .tx
            .send(Command::Cancel {
                recording_was_active,
            })
            .is_err()
        {
            warn!("Transcription coordinator channel closed");
        }
    }

    pub fn notify_processing_finished(&self) {
        if self.tx.send(Command::ProcessingFinished).is_err() {
            warn!("Transcription coordinator channel closed");
        }
    }
}

fn start(app: &AppHandle, stage: &mut Stage, binding_id: &str, hotkey_string: &str) {
    let Some(action) = ACTION_MAP.get(binding_id) else {
        warn!("No action in ACTION_MAP for '{binding_id}'");
        return;
    };
    action.start(app, binding_id, hotkey_string);
    if app
        .try_state::<Arc<AudioRecordingManager>>()
        .map_or(false, |a| a.is_recording())
    {
        *stage = Stage::Recording(binding_id.to_string());
    } else {
        debug!("Start for '{binding_id}' did not begin recording; staying idle");
    }
}

fn stop(app: &AppHandle, stage: &mut Stage, binding_id: &str, hotkey_string: &str) {
    let Some(action) = ACTION_MAP.get(binding_id) else {
        warn!("No action in ACTION_MAP for '{binding_id}'");
        return;
    };
    action.stop(app, binding_id, hotkey_string);
    *stage = Stage::Processing;
}

fn cancel_recording(app: &AppHandle, stage: &mut Stage) {
    if let Stage::Recording(binding_id) = stage {
        debug!("Cancelling recording for '{binding_id}'");
        // 调用 cancel action
        if let Some(action) = ACTION_MAP.get("cancel") {
            action.start(app, "cancel", "cancel");
        }
        *stage = Stage::Idle;
    }
}
