use std::thread;
use std::time::Duration;

/// Stages in the agent processing pipeline that benefit from deliberate thinking time.
#[derive(Debug, Clone, Copy)]
pub enum ProcessingStage {
    RequirementsExtraction,
    Planning,
    CodeOutline,
    CodeDraft,
    CodeRefinement,
    StaticReview,
    Debugging,
    TestGeneration,
    TestExecution,
    FinalValidation,
}

impl ProcessingStage {
    /// Returns a human-readable label for the stage.
    pub fn label(&self) -> &str {
        match self {
            ProcessingStage::RequirementsExtraction => "Extracting requirements",
            ProcessingStage::Planning => "Planning implementation steps",
            ProcessingStage::CodeOutline => "Outlining code structure",
            ProcessingStage::CodeDraft => "Drafting implementation",
            ProcessingStage::CodeRefinement => "Refining code and adding edge cases",
            ProcessingStage::StaticReview => "Reviewing code quality",
            ProcessingStage::Debugging => "Fixing identified issues",
            ProcessingStage::TestGeneration => "Generating test cases",
            ProcessingStage::TestExecution => "Running test cases",
            ProcessingStage::FinalValidation => "Validating against requirements",
        }
    }

    /// Returns the ANSI color code for the stage.
    pub fn color(&self) -> &str {
        match self {
            ProcessingStage::RequirementsExtraction | ProcessingStage::Planning => "\x1b[36m", // Cyan
            ProcessingStage::CodeOutline | ProcessingStage::CodeDraft | ProcessingStage::CodeRefinement => "\x1b[33m", // Yellow
            ProcessingStage::StaticReview | ProcessingStage::Debugging => "\x1b[35m", // Magenta
            ProcessingStage::TestGeneration | ProcessingStage::TestExecution | ProcessingStage::FinalValidation => "\x1b[32m", // Green
        }
    }
}

/// A timer that shows progress during a deliberate thinking stage.
pub struct ThinkingTimer {
    pub stage: ProcessingStage,
    pub duration_secs: u32,
}

impl ThinkingTimer {
    /// Creates a new thinking timer for the given stage and duration.
    pub fn new(stage: ProcessingStage, duration_secs: u32) -> Self {
        ThinkingTimer { stage, duration_secs }
    }

    /// Starts the timer and displays progress every second.
    pub fn start(&self) {
        let color = self.stage.color();
        let label = self.stage.label();
        let reset = "\x1b[0m";

        for elapsed in 1..=self.duration_secs {
            print!("\r  {color}[THINKING]{reset} {} ... {}/{} sec", label, elapsed, self.duration_secs);
            std::io::Write::flush(&mut std::io::stdout()).expect("Failed to flush stdout");
            thread::sleep(Duration::from_secs(1));
        }
        println!(); // Move to next line after completion
    }
}
