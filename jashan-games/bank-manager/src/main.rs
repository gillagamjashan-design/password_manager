use ggez::event::{self, EventHandler};
use ggez::graphics::{self, Color, DrawMode, Mesh, Rect, Text};
use ggez::{Context, GameResult};
use rand::Rng;
use std::collections::VecDeque;

const WINDOW_WIDTH: f32 = 1280.0;
const WINDOW_HEIGHT: f32 = 720.0;

#[derive(Debug, Clone)]
enum TransactionType {
    Deposit(f32),
    Withdrawal(f32),
    LoanRequest(f32),
    SuspiciousActivity(String),
}

#[derive(Debug, Clone)]
struct Customer {
    name: String,
    account_balance: f32,
    credit_score: i32,
    transaction: TransactionType,
    is_fraudulent: bool,
    patience: f32,
}

impl Customer {
    fn new_random(rng: &mut rand::rngs::ThreadRng, _day: u32) -> Self {
        let first_names = vec!["John", "Emma", "Michael", "Sophia", "David", "Olivia", "James", "Ava"];
        let last_names = vec!["Smith", "Johnson", "Williams", "Brown", "Jones", "Garcia", "Miller"];

        let name = format!(
            "{} {}",
            first_names[rng.gen_range(0..first_names.len())],
            last_names[rng.gen_range(0..last_names.len())]
        );

        let account_balance = rng.gen_range(100.0..50000.0);
        let credit_score = rng.gen_range(300..850);

        let transaction = match rng.gen_range(0..4) {
            0 => TransactionType::Deposit(rng.gen_range(50.0..5000.0)),
            1 => TransactionType::Withdrawal(rng.gen_range(50.0..2000.0)),
            2 => TransactionType::LoanRequest(rng.gen_range(1000.0..50000.0)),
            _ => TransactionType::SuspiciousActivity("Large cash deposit".to_string()),
        };

        let is_fraudulent = rng.gen_range(0..100) < 15;

        Customer {
            name,
            account_balance,
            credit_score,
            transaction,
            is_fraudulent,
            patience: 100.0,
        }
    }
}

struct GameState {
    customer_queue: VecDeque<Customer>,
    current_customer: Option<Customer>,
    bank_funds: f32,
    player_level: u32,
    experience: u32,
    day: u32,
    score: i32,
    total_customers_served: u32,
    correct_fraud_detections: u32,
    rng: rand::rngs::ThreadRng,
}

impl GameState {
    fn new() -> Self {
        let mut state = GameState {
            customer_queue: VecDeque::new(),
            current_customer: None,
            bank_funds: 100000.0,
            player_level: 1,
            experience: 0,
            day: 1,
            score: 0,
            total_customers_served: 0,
            correct_fraud_detections: 0,
            rng: rand::thread_rng(),
        };

        state.spawn_customers(3);
        state.next_customer();
        state
    }

    fn spawn_customers(&mut self, count: usize) {
        for _ in 0..count {
            let customer = Customer::new_random(&mut self.rng, self.day);
            self.customer_queue.push_back(customer);
        }
    }

    fn next_customer(&mut self) {
        self.current_customer = self.customer_queue.pop_front();

        if self.customer_queue.len() < 2 {
            self.spawn_customers(2);
        }
    }

    fn approve_transaction(&mut self) {
        if let Some(customer) = &self.current_customer {
            match &customer.transaction {
                TransactionType::Deposit(amount) => {
                    self.bank_funds += amount;
                    self.score += 10;
                    self.gain_experience(5);
                }
                TransactionType::Withdrawal(amount) => {
                    if customer.account_balance >= *amount {
                        self.bank_funds -= amount;
                        self.score += 10;
                        self.gain_experience(5);
                    } else {
                        self.score -= 20;
                    }
                }
                TransactionType::LoanRequest(amount) => {
                    if customer.credit_score > 650 {
                        self.bank_funds -= amount;
                        self.score += 50;
                        self.gain_experience(25);
                    } else {
                        self.score -= 30;
                    }
                }
                TransactionType::SuspiciousActivity(_) => {
                    if customer.is_fraudulent {
                        self.score -= 100;
                    } else {
                        self.score += 20;
                        self.gain_experience(10);
                    }
                }
            }

            self.total_customers_served += 1;
            self.next_customer();
        }
    }

    fn deny_transaction(&mut self) {
        if let Some(customer) = &self.current_customer {
            match &customer.transaction {
                TransactionType::SuspiciousActivity(_) => {
                    if customer.is_fraudulent {
                        self.score += 200;
                        self.correct_fraud_detections += 1;
                        self.gain_experience(50);
                    } else {
                        self.score -= 50;
                    }
                }
                _ => {
                    self.score -= 10;
                }
            }

            self.next_customer();
        }
    }

    fn gain_experience(&mut self, amount: u32) {
        self.experience += amount;
        let exp_needed = self.player_level * 100;

        if self.experience >= exp_needed {
            self.player_level += 1;
            self.experience = 0;
            self.score += 500;
        }
    }
}

impl EventHandler for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        if let Some(customer) = &mut self.current_customer {
            customer.patience -= 0.1;

            if customer.patience <= 0.0 {
                self.score -= 50;
                self.next_customer();
            }
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::from_rgb(240, 240, 245));

        let header_bg = Mesh::new_rectangle(
            ctx,
            DrawMode::fill(),
            Rect::new(0.0, 0.0, WINDOW_WIDTH, 80.0),
            Color::from_rgb(30, 50, 100),
        )?;
        canvas.draw(&header_bg, graphics::DrawParam::default());

        let title = Text::new(format!(
            "Bank Manager - Day {} | Level {} | Score: {} | Bank Funds: ${:.0}",
            self.day, self.player_level, self.score, self.bank_funds
        ));
        canvas.draw(
            &title,
            graphics::DrawParam::default()
                .dest([20.0, 25.0])
                .color(Color::WHITE),
        );

        if let Some(customer) = &self.current_customer {
            let customer_bg = Mesh::new_rectangle(
                ctx,
                DrawMode::fill(),
                Rect::new(50.0, 120.0, 500.0, 450.0),
                Color::from_rgb(255, 255, 255),
            )?;
            canvas.draw(&customer_bg, graphics::DrawParam::default());

            let customer_border = Mesh::new_rectangle(
                ctx,
                DrawMode::stroke(3.0),
                Rect::new(50.0, 120.0, 500.0, 450.0),
                Color::from_rgb(30, 50, 100),
            )?;
            canvas.draw(&customer_border, graphics::DrawParam::default());

            let mut y_pos = 150.0;
            let info_texts = vec![
                format!("CUSTOMER: {}", customer.name),
                format!("Account Balance: ${:.2}", customer.account_balance),
                format!("Credit Score: {}", customer.credit_score),
                format!(""),
                format!("REQUEST:"),
                match &customer.transaction {
                    TransactionType::Deposit(amt) => format!("Deposit ${:.2}", amt),
                    TransactionType::Withdrawal(amt) => format!("Withdraw ${:.2}", amt),
                    TransactionType::LoanRequest(amt) => format!("Loan Application: ${:.2}", amt),
                    TransactionType::SuspiciousActivity(desc) => format!("⚠️ ALERT: {}", desc),
                },
                format!(""),
                format!("Patience: {:.0}%", customer.patience),
            ];

            for text_str in info_texts {
                let text = Text::new(text_str);
                canvas.draw(
                    &text,
                    graphics::DrawParam::default()
                        .dest([70.0, y_pos])
                        .color(Color::BLACK),
                );
                y_pos += 35.0;
            }

            let approve_btn = Mesh::new_rectangle(
                ctx,
                DrawMode::fill(),
                Rect::new(650.0, 250.0, 200.0, 60.0),
                Color::from_rgb(50, 150, 50),
            )?;
            canvas.draw(&approve_btn, graphics::DrawParam::default());

            let approve_text = Text::new("APPROVE (A)");
            canvas.draw(
                &approve_text,
                graphics::DrawParam::default()
                    .dest([685.0, 270.0])
                    .color(Color::WHITE),
            );

            let deny_btn = Mesh::new_rectangle(
                ctx,
                DrawMode::fill(),
                Rect::new(650.0, 350.0, 200.0, 60.0),
                Color::from_rgb(150, 50, 50),
            )?;
            canvas.draw(&deny_btn, graphics::DrawParam::default());

            let deny_text = Text::new("DENY (D)");
            canvas.draw(
                &deny_text,
                graphics::DrawParam::default()
                    .dest([700.0, 370.0])
                    .color(Color::WHITE),
            );
        } else {
            let no_customer_text = Text::new("No customers waiting...");
            canvas.draw(
                &no_customer_text,
                graphics::DrawParam::default()
                    .dest([50.0, 300.0])
                    .color(Color::BLACK),
            );
        }

        let queue_bg = Mesh::new_rectangle(
            ctx,
            DrawMode::fill(),
            Rect::new(920.0, 120.0, 320.0, 450.0),
            Color::from_rgb(245, 245, 250),
        )?;
        canvas.draw(&queue_bg, graphics::DrawParam::default());

        let queue_title = Text::new("QUEUE");
        canvas.draw(
            &queue_title,
            graphics::DrawParam::default()
                .dest([940.0, 140.0])
                .color(Color::BLACK),
        );

        let mut queue_y = 180.0;
        for (i, customer) in self.customer_queue.iter().take(8).enumerate() {
            let queue_text = Text::new(format!("{}. {}", i + 1, customer.name));
            canvas.draw(
                &queue_text,
                graphics::DrawParam::default()
                    .dest([940.0, queue_y])
                    .color(Color::from_rgb(60, 60, 60)),
            );
            queue_y += 35.0;
        }

        let stats_bg = Mesh::new_rectangle(
            ctx,
            DrawMode::fill(),
            Rect::new(50.0, 600.0, 800.0, 80.0),
            Color::from_rgb(230, 240, 250),
        )?;
        canvas.draw(&stats_bg, graphics::DrawParam::default());

        let stats_text = Text::new(format!(
            "Customers Served: {}  |  Fraud Detected: {}  |  Experience: {}/{}",
            self.total_customers_served,
            self.correct_fraud_detections,
            self.experience,
            self.player_level * 100
        ));
        canvas.draw(
            &stats_text,
            graphics::DrawParam::default()
                .dest([70.0, 630.0])
                .color(Color::BLACK),
        );

        canvas.finish(ctx)?;
        Ok(())
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        input: ggez::input::keyboard::KeyInput,
        _repeated: bool,
    ) -> GameResult {
        if let Some(keycode) = input.keycode {
            match keycode {
                ggez::input::keyboard::KeyCode::A => {
                    self.approve_transaction();
                }
                ggez::input::keyboard::KeyCode::D => {
                    self.deny_transaction();
                }
                _ => {}
            }
        }
        Ok(())
    }
}

fn main() -> GameResult {
    let (ctx, event_loop) = ggez::ContextBuilder::new("bank_manager", "YourName")
        .window_setup(ggez::conf::WindowSetup::default().title("Bank Manager Game"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(WINDOW_WIDTH, WINDOW_HEIGHT))
        .build()?;

    let state = GameState::new();
    event::run(ctx, event_loop, state)
}
