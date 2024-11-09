use std::io;

use ggez;
use ggez::event;
use ggez::graphics;

use ggez::nalgebra as na;
use ggez::{Context, GameResult};
use rand::{self, thread_rng, Rng};
use serde::Deserialize;
use tokio::sync::mpsc::{Receiver, Sender};

const PADDING: f32 = 40.0;
const MIDDLE_LINE_W: f32 = 2.0;
const RACKET_HEIGHT: f32 = 100.0;
const RACKET_WIDTH: f32 = 20.0;
const RACKET_WIDTH_HALF: f32 = RACKET_WIDTH * 0.5;
const RACKET_HEIGHT_HALF: f32 = RACKET_HEIGHT * 0.5;
const BALL_SIZE: f32 = 30.0;
const BALL_SIZE_HALF: f32 = BALL_SIZE * 0.5;
const BALL_SPEED: f32 = 200.0;

pub fn randomize_vec(vec: &mut na::Vector2<f32>, x: f32, y: f32) {
    let mut rng = thread_rng();
    vec.x = match rng.gen_bool(0.5) {
        true => x,
        false => -x,
    };
    vec.y = match rng.gen_bool(0.5) {
        true => y,
        false => -y,
    };
}

#[derive(Deserialize, Debug)]
pub struct GameData {
    pos_player1: f32,
    pos_player2: f32,
    pos_ball_x: f32,
    pos_ball_y: f32,
    score_player_1: f32,
    score_player_2: f32,
}

pub struct MainState {
    player_1_pos: na::Point2<f32>,
    player_2_pos: na::Point2<f32>,
    ball_pos: na::Point2<f32>,
    ball_vel: na::Vector2<f32>,
    player_1_score: f32,
    player_2_score: f32,
    rx: Receiver<GameData>,
}

impl MainState {
    pub fn new(ctx: &mut Context, rx: Receiver<GameData>) -> Self {
        let (screen_w, screen_h) = graphics::drawable_size(ctx);
        let (screen_w_half, screen_h_half) = (screen_w * 0.5, screen_h * 0.5);

        let mut ball_vel = na::Vector2::new(0.0, 0.0);
        randomize_vec(&mut ball_vel, BALL_SPEED, BALL_SPEED);

        MainState {
            player_1_pos: na::Point2::new(RACKET_WIDTH_HALF + PADDING, screen_h_half),
            player_2_pos: na::Point2::new(screen_w - RACKET_WIDTH_HALF - PADDING, screen_h_half),
            ball_pos: na::Point2::new(screen_w_half, screen_h_half),
            ball_vel,
            player_1_score: 0.0,
            player_2_score: 0.0,
            rx,
        }
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let (screen_w, screen_h) = graphics::drawable_size(ctx);
        if let Ok(data) = self.rx.try_recv() {
            self.player_1_pos.y = data.pos_player1;
            self.player_1_score = data.score_player_1;
            self.player_2_pos.y = data.pos_player2;
            self.player_2_score = data.score_player_2;
            self.ball_pos.x = data.pos_ball_x;
            self.ball_pos.y = data.pos_ball_y;
        }

        if self.ball_pos.x < 0.0 {
            self.ball_pos.x = screen_w * 0.5;
            self.ball_pos.y = screen_h * 0.5;
            randomize_vec(&mut self.ball_vel, BALL_SPEED, BALL_SPEED);
            self.player_2_score += 1.0;
        }
        if self.ball_pos.x > screen_w {
            self.ball_pos.x = screen_w * 0.5;
            self.ball_pos.y = screen_h * 0.5;
            randomize_vec(&mut self.ball_vel, BALL_SPEED, BALL_SPEED);
            self.player_1_score += 1.0;
        }

        if self.ball_pos.y < BALL_SIZE_HALF {
            self.ball_pos.y = BALL_SIZE_HALF;
            self.ball_vel.y = self.ball_vel.y.abs();
        } else if self.ball_pos.y > screen_h - BALL_SIZE_HALF {
            self.ball_pos.y = screen_h - BALL_SIZE_HALF;
            self.ball_vel.y = -self.ball_vel.y.abs();
        }

        let intersects_player_1 = self.ball_pos.x - BALL_SIZE_HALF
            < self.player_1_pos.x + RACKET_WIDTH_HALF
            && self.ball_pos.x + BALL_SIZE_HALF > self.player_1_pos.x - RACKET_WIDTH_HALF
            && self.ball_pos.y - BALL_SIZE_HALF < self.player_1_pos.y + RACKET_HEIGHT_HALF
            && self.ball_pos.y + BALL_SIZE_HALF > self.player_1_pos.y - RACKET_HEIGHT_HALF;

        if intersects_player_1 {
            self.ball_vel.x = self.ball_vel.x.abs();
        }
        let intersects_player_2 = self.ball_pos.x - BALL_SIZE_HALF
            < self.player_2_pos.x + RACKET_WIDTH_HALF
            && self.ball_pos.x + BALL_SIZE_HALF > self.player_2_pos.x - RACKET_WIDTH_HALF
            && self.ball_pos.y - BALL_SIZE_HALF < self.player_2_pos.y + RACKET_HEIGHT_HALF
            && self.ball_pos.y + BALL_SIZE_HALF > self.player_2_pos.y - RACKET_HEIGHT_HALF;

        if intersects_player_2 {
            self.ball_vel.x = -self.ball_vel.x.abs();
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::BLACK);

        let racket_rect = graphics::Rect::new(
            -RACKET_WIDTH_HALF,
            -RACKET_HEIGHT_HALF,
            RACKET_WIDTH,
            RACKET_HEIGHT,
        );
        let racket_mesh = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            racket_rect,
            graphics::WHITE,
        )?;

        let ball_rect = graphics::Rect::new(-BALL_SIZE_HALF, -BALL_SIZE_HALF, BALL_SIZE, BALL_SIZE);
        let ball_mesh = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            ball_rect,
            graphics::WHITE,
        )?;

        let screen_h = graphics::drawable_size(ctx).1;
        let middle_rect = graphics::Rect::new(-MIDDLE_LINE_W * 0.5, 0.0, MIDDLE_LINE_W, screen_h);
        let middle_mesh = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            middle_rect,
            graphics::WHITE,
        )?;

        let mut draw_param = graphics::DrawParam::default();

        let screen_middle_x = graphics::drawable_size(ctx).0 * 0.5;
        draw_param.dest = [screen_middle_x, 0.0].into();
        graphics::draw(ctx, &middle_mesh, draw_param)?;

        draw_param.dest = self.player_1_pos.into();
        graphics::draw(ctx, &racket_mesh, draw_param)?;

        draw_param.dest = self.player_2_pos.into();
        graphics::draw(ctx, &racket_mesh, draw_param)?;

        draw_param.dest = self.ball_pos.into();
        graphics::draw(ctx, &ball_mesh, draw_param)?;

        let score_text = graphics::Text::new(format!(
            "{}         {}",
            self.player_1_score, self.player_2_score
        ));
        let screen_w = graphics::drawable_size(ctx).0;
        let screen_w_half = screen_w * 0.5;

        let mut score_pos = na::Point2::new(screen_w_half, 40.0);
        let (score_text_w, score_text_h) = score_text.dimensions(ctx);
        score_pos -= na::Vector2::new(score_text_w as f32 * 0.5, score_text_h as f32 * 0.5);
        draw_param.dest = score_pos.into();

        graphics::draw(ctx, &score_text, draw_param)?;

        graphics::present(ctx)?;
        Ok(())
    }
}

pub fn game_pingpong_run(rx: Receiver<GameData>) {
    let cb: ggez::ContextBuilder = ggez::ContextBuilder::new("pong", "TanTan");
    let (mut ctx, mut event_loop) = cb.build().unwrap();
    graphics::set_window_title(&ctx, "Client device");
    let mut state = MainState::new(&mut ctx, rx);

    event::run(&mut ctx, &mut event_loop, &mut state).expect("Cannot run");
}

pub async fn pingpong_update(tx: Sender<GameData>, data: Vec<u8>) -> Result<(), io::Error> {
    let game_data: GameData = if let Ok(data) =  serde_json::from_slice(&data){
         data
    }else{
        println!("Not true");
        return Ok(());
    };
    tx.send(game_data).await.expect("Can't send to update game");
    Ok(())
}
