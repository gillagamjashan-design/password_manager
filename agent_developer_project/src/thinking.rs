// ============================================================
// thinking.rs: Deliberate processing delays and progress indicators
// ============================================================
//
// This module provides authentic thinking time visualization for agents.
// Each agent stage shows real-time progress with live second-by-second updates.

use std::io::{self, Write};
use std::thread;
use std::time::Duration;

// ANSI color/style helpers
const RESET: &str = "\x1b[0m";
const BOLD: &str = "\x1b[1m";
const DIM: &str = "\x1b[2m";
const CYAN: &str = "\x1b[36m";
const YELLOW: &str = "\x1b[33m";
const GREEN: &str = "\x1b[32m";
const MAGENTA: &str = "\x1b[35m";

/// Represents the different processing stages agents go through
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
    /// Returns a human-readable label for the stage
    pub fn label(&self) -> &str {
        match self {
            ProcessingStage::RequirementsExtraction => "Extracting requirements",
            ProcessingStage::Planning => "Planning implementation",
            ProcessingStage::CodeOutline => "Creating code outline",
            ProcessingStage::CodeDraft => "Drafting code implementation",
            ProcessingStage::CodeRefinement => "Refining and polishing code",
            ProcessingStage::StaticReview => "Reviewing code quality",
            ProcessingStage::Debugging => "Analyzing and fixing issues",
            ProcessingStage::TestGeneration => "Generating test cases",
            ProcessingStage::TestExecution => "Running tests",
            ProcessingStage::FinalValidation => "Validating against requirements",
        }
    }

    /// Returns the ANSI color code for the stage
    pub fn color(&self) -> &str {
        match self {
            ProcessingStage::RequirementsExtraction => CYAN,
            ProcessingStage::Planning => CYAN,
            ProcessingStage::CodeOutline => YELLOW,
            ProcessingStage::CodeDraft => YELLOW,
            ProcessingStage::CodeRefinement => YELLOW,
            ProcessingStage::StaticReview => MAGENTA,
            ProcessingStage::Debugging => MAGENTA,
            ProcessingStage::TestGeneration => GREEN,
            ProcessingStage::TestExecution => GREEN,
            ProcessingStage::FinalValidation => GREEN,
        }
    }
}

/// A timer that shows real-time progress for a processing stage
pub struct ThinkingTimer {
    stage: ProcessingStage,
    duration_secs: u64,
}

impl ThinkingTimer {
    /// Create a new thinking timer for a specific stage and duration
    pub fn new(stage: ProcessingStage, duration_secs: u64) -> Self {
        ThinkingTimer {
            stage,
            duration_secs,
        }
    }

    /// Start the timer and show live progress updates
    pub fn start(&self) {
        let color = self.stage.color();
        let label = self.stage.label();

        for i in 1..=self.duration_secs {
            // Clear the current line and move cursor to start
            print!("\r\x1b[K");

            // Print progress with elapsed/total time
            print!(
                "  {}{}...{} {}{}/{} sec{}",
                color,
                label,
                RESET,
                DIM,
                i,
                self.duration_secs,
                RESET
            );

            io::stdout().flush().expect("Failed to flush stdout");

            // Sleep for 1 second
            thread::sleep(Duration::from_secs(1));
        }

        // Clear the line after completion
        print!("\r\x1b[K");
        io::stdout().flush().expect("Failed to flush stdout");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stage_labels() {
        assert_eq!(
            ProcessingStage::RequirementsExtraction.label(),
            "Extracting requirements"
        );
        assert_eq!(
            ProcessingStage::Planning.label(),
            "Planning implementation"
        );
        assert_eq!(
            ProcessingStage::CodeOutline.label(),
            "Creating code outline"
        );
        assert_eq!(
            ProcessingStage::CodeDraft.label(),
            "Drafting code implementation"
        );
        assert_eq!(
            ProcessingStage::CodeRefinement.label(),
            "Refining and polishing code"
        );
        assert_eq!(
            ProcessingStage::StaticReview.label(),
            "Reviewing code quality"
        );
        assert_eq!(
            ProcessingStage::Debugging.label(),
            "Analyzing and fixing issues"
        );
        assert_eq!(
            ProcessingStage::TestGeneration.label(),
            "Generating test cases"
        );
        assert_eq!(
            ProcessingStage::TestExecution.label(),
            "Running tests"
        );
        assert_eq!(
            ProcessingStage::FinalValidation.label(),
            "Validating against requirements"
        );
    }

    #[test]
    fn test_stage_colors() {
        // Planning stages should be cyan
        assert_eq!(ProcessingStage::RequirementsExtraction.color(), CYAN);
        assert_eq!(ProcessingStage::Planning.color(), CYAN);

        // Coding stages should be yellow
        assert_eq!(ProcessingStage::CodeOutline.color(), YELLOW);
        assert_eq!(ProcessingStage::CodeDraft.color(), YELLOW);
        assert_eq!(ProcessingStage::CodeRefinement.color(), YELLOW);

        // Review/debug stages should be magenta
        assert_eq!(ProcessingStage::StaticReview.color(), MAGENTA);
        assert_eq!(ProcessingStage::Debugging.color(), MAGENTA);

        // Validation stages should be green
        assert_eq!(ProcessingStage::TestGeneration.color(), GREEN);
        assert_eq!(ProcessingStage::TestExecution.color(), GREEN);
        assert_eq!(ProcessingStage::FinalValidation.color(), GREEN);
    }

    #[test]
    fn test_timer_creation() {
        let timer = ThinkingTimer::new(ProcessingStage::Planning, 10);
        assert_eq!(timer.duration_secs, 10);
    }
}
